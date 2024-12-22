use genai::chat::printer::print_chat_stream;
use genai::chat::{ChatMessage, ChatRequest};
use genai::Client;

// const MODEL_OPENAI: &str = "gpt-4o-mini";
// const MODEL_ANTHROPIC: &str = "claude-3-haiku-20240307";
// const MODEL_COHERE: &str = "command-light";
// const MODEL_GEMINI: &str = "gemini-1.5-flash-latest";
// const MODEL_GROQ: &str = "gemma-7b-it";
// const MODEL_OLLAMA: &str = "gemma:2b"; // sh: `ollama pull gemma:2b`
// const MODEL_XAI: &str = "grok-beta";

// const MODEL_AND_KEY_ENV_NAME_LIST: &[(&str, &str)] = &[
//     // -- de/activate models/providers
//     (MODEL_OPENAI, "OPENAI_API_KEY"),
//     (MODEL_ANTHROPIC, "ANTHROPIC_API_KEY"),
//     (MODEL_COHERE, "COHERE_API_KEY"),
//     (MODEL_GEMINI, "GEMINI_API_KEY"),
//     (MODEL_GROQ, "GROQ_API_KEY"),
//     (MODEL_XAI, "XAI_API_KEY"),
//     (MODEL_OLLAMA, ""),
// ];
//
// const MODEL: &str = "neural-chat:latest";
const MODEL: &str = "gpt-4o-mini";
// const MODEL: &str = "llama3.3";
// const MODEL: &str = "claude-3-haiku-20240307";

// NOTE: Model to AdapterKind (AI Provider) type mapping rule
//  - starts_with "gpt"      -> OpenAI
//  - starts_with "claude"   -> Anthropic
//  - starts_with "command"  -> Cohere
//  - starts_with "gemini"   -> Gemini
//  - model in Groq models   -> Groq
//  - For anything else      -> Ollama
pub async fn getmessage(
    input: &str,
    note: &str,
    model: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let question = format!("{}: {}", input, note);

    // -- Build the new client with this client_config
    let client = Client::default();

    let mut chat_req = ChatRequest::default().with_system("Sie sind hier, um mir bei der Organisation meiner Notizen und Todos zu helfen und Informationen über die notizen zu liefern, die ich Ihnen sende. Die notizen werden immer in HTML-Code geschrieben und ich möchte, dass Sie auch in HTML antworten. Das bedeutet, statt einfach ANTWORT zu schreiben, würden Sie <p>ANTWORT</p> schreiben und so weiter, außer wenn anders gewünscht.");

    chat_req = chat_req.append_message(ChatMessage::user(question));

    let chat_res = client
        .exec_chat_stream(MODEL, chat_req.clone(), None)
        .await?;

    let assistant_answer = print_chat_stream(chat_res, None).await?;

    // chat_req = chat_req.append_message(ChatMessage::assistant(assistant_answer.clone()));

    Ok(assistant_answer)
}
