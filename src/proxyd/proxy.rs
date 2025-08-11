use tracing::{Level, debug, span, warn};

use crate::proxyd::{
    config::{OpenAICompatible, ProxyConfig},
    models::{
        AnthropicContent, AnthropicContentBlock, AnthropicRequest, AnthropicResponse,
        AnthropicUsage, OpenAIChoice, OpenAIContent, OpenAIContentPart, OpenAIImageUrl,
        OpenAIMessage, OpenAIRequest, OpenAIResponse,
    },
};

pub struct ProxyHandler {
    openai_provider: Vec<OpenAICompatible>,
    proxy_config: ProxyConfig,
}

fn convert_content(anthropic_content: AnthropicContent) -> Result<OpenAIContent, String> {
    match anthropic_content {
        AnthropicContent::String(text) => Ok(OpenAIContent::String(text)),
        AnthropicContent::Blocks(blocks) => {
            let mut content_parts = Vec::new();

            for block in blocks {
                match block {
                    AnthropicContentBlock::Text { text } => {
                        content_parts.push(OpenAIContentPart::Text { text });
                    }
                    AnthropicContentBlock::Image { source } => {
                        let data_url = format!("data:{};base64,{}", source.media_type, source.data);

                        content_parts.push(OpenAIContentPart::ImageUrl {
                            image_url: OpenAIImageUrl {
                                url: data_url,
                                detail: Some("auto".to_string()),
                            },
                        });
                    }
                }
            }

            Ok(OpenAIContent::Parts(content_parts))
        }
    }
}

impl ProxyHandler {
    pub fn new(openai_provider: Vec<OpenAICompatible>, proxy_config: ProxyConfig) -> Self {
        Self {
            openai_provider,
            proxy_config,
        }
    }
    pub fn anthropic_to_openai(anthropic_req: AnthropicRequest) -> Result<OpenAIRequest, String> {
        let _span = span!(Level::DEBUG, "convert_anthropic_to_openai").entered();

        debug!(
            model = %anthropic_req.model,
            message_count = anthropic_req.messages.len(),
            "Converting Anthropic request to OpenAI"
        );

        let mut openai_messages = Vec::new();

        // Add System
        if let Some(system_content) = anthropic_req.system {
            openai_messages.push(OpenAIMessage {
                role: "system".to_string(),
                content: OpenAIContent::String(system_content),
                name: None,
            });
        }

        for message in anthropic_req.messages {
            let openai_content = convert_content(message.content)?;

            openai_messages.push(OpenAIMessage {
                role: message.role,
                content: openai_content,
                name: None,
            });
        }

        let stop = match anthropic_req.stop_sequences {
            Some(sequences) if sequences.len() == 1 => {
                Some(serde_json::Value::String(sequences[0].clone()))
            }
            Some(sequences) if sequences.len() > 1 => Some(serde_json::Value::Array(
                sequences
                    .into_iter()
                    .map(serde_json::Value::String)
                    .collect(),
            )),
            _ => None,
        };

        Ok(OpenAIRequest {
            model: anthropic_req.model,
            messages: openai_messages,
            temperature: anthropic_req.temperature,
            top_p: anthropic_req.top_p,
            max_tokens: Some(anthropic_req.max_tokens),
            stop,
            stream: anthropic_req.stream,
            user: None,
        })
    }

    pub fn openai_to_anthropic(
        openai_resp: OpenAIResponse,
        original_model: &str,
    ) -> AnthropicResponse {
        let _span = span!(Level::DEBUG, "convert_openai_to_anthropic").entered();

        let choice = openai_resp.choices.into_iter().next().unwrap_or_else(|| {
            warn!("No choices in OpenAI response, creating empty choice");
            OpenAIChoice {
                index: 0,
                message: OpenAIMessage {
                    role: "assistant".to_string(),
                    content: OpenAIContent::String("".to_string()),
                    name: None,
                },
                finish_reason: "stop".to_string(),
            }
        });

        let content_blocks =
            match choice.message.content {
                OpenAIContent::String(text) => {
                    vec![AnthropicContentBlock::Text { text }]
                }
                OpenAIContent::Parts(parts) => {
                    parts
                .into_iter()
                .filter_map(|part| match part {
                    OpenAIContentPart::Text { text } => Some(AnthropicContentBlock::Text { text }),
                    OpenAIContentPart::ImageUrl { image_url } => {
                        // TODO: implement image result convert.
                        debug!(url = %image_url.url, "Skipping image in response conversion");
                        None
                    }
                })
                .collect()
                }
            };

        let stop_reason = match choice.finish_reason.as_str() {
            "stop" => "end_turn",
            "length" => "max_tokens",
            "content_filter" => "end_turn",
            _ => "end_turn",
        }
        .to_string();

        debug!(
            response_id = %openai_resp.id,
            content_blocks = content_blocks.len(),
            stop_reason = %stop_reason,
            "Response conversion completed"
        );

        AnthropicResponse {
            id: openai_resp.id,
            response_type: "message".to_string(),
            role: "assistant".to_string(),
            content: content_blocks,
            model: original_model.to_string(),
            stop_reason,
            stop_sequence: None,
            usage: AnthropicUsage {
                input_tokens: openai_resp.usage.prompt_tokens,
                output_tokens: openai_resp.usage.completion_tokens,
            },
        }
    }
}
