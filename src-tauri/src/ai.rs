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

    let mut chat_req = ChatRequest::default().with_system("Sie sind hier, um mir bei der Organisation meiner Notizen und Todos zu helfen und Informationen über die notizen zu liefern, die ich Ihnen sende. Die notizen werden immer in HTML-Code geschrieben und ich möchte, dass Sie auch in HTML antworten. Das bedeutet, statt einfach ANTWORT zu schreiben, würden Sie <p>ANTWORT</p> schreiben und so weiter, außer wenn anders gewünscht.
    Manchmal brauche ich z.B. einen Change Request, das heißt wenn ich dich nach einem Change Request frage nimm die Informationen aus meiner Notiz und erstelle einen Change Request nach diesem beispiel:

    CR Titel:
        Sachkontospezifische Ablage

    Beschreibung der Änderungen:
        Bei der KundenName werden die Rechnungen aktuell über die Standard ELO Invoice Ablage abgelegt und sind somit unter dem dementsprechenden Jahr, sowie dem entsprechenden Kreditoren abgelegt.
        Ebenfalls gibt es bei der KundenName einen Pfad im Archiv, unter welchem alle vorhandenen Buchungskonten angelegt sind. Diese Buchungskonten sollen in Zukunft im Rechnungsfreigabeprozess unter den Positionen im Reiter „Sachkonto“ auswählbar sein, weiterhin sollen die Rechnungen in die jeweiligen Ordner der Buchungskonten referenziert werden.  
        Da aktuell schon eine Vielzahl an Sachkonten mit der Maske „Freie Eingabe“ angelegt und diese Ordner mit Inhalt befüllt wurden, ist zusätzlich eine Migration der Masken in eine neue Maske „Konto“ benötigt. Die Konten erhalten eine Maske mit der Bezeichnung „Konto“ und den Datenfeldern „Kontonummer“, „Kontobezeichnung“ und „Ausprägung“. Die Ausprägung bezeichnet die Art des Kontos, also „Sachkonto“, „GUV-Konto“, „CPD- und Hilfskonto“ „Nostrokonto“ oder „Externes Konto“. Ebenfalls handelt es sich hierbei um Indexfelder, es kann in der Suche also auch nach genau diesen Feldern gesucht werden.
        Neue Sachkonten werden aktuell in eine csv Datei eingefügt und per ELO-Import ins Archiv geholt. Für die Anlage eines neuen Geschäftsjahres müssen alle Konten nochmal neu importiert werden.
        In Zukunft wird dies so ablaufen, dass neue Konten, sowie neue Jahre der Konten über einen Button angelegt werden. Für ein neues Konto wird der Button „Neues Konto“ ausgewählt, es müssen Kontonummer + Bezeichnung eingetragen werden und die Ausprägung ausgewählt werden. Ein neues Jahr wird entweder dann angelegt, wenn eine neue Rechnung ins dementsprechende Konto referenziert wird, oder wenn der Button „neues Geschäftsjahr anlegen“ gewählt wird.Wird in einer Rechnung ein Sachkonto eingegeben, welches noch nicht in der Datenbank existiert, erscheint die Meldung „Sachkonto nicht vorhanden, bitte um Anlage“.
        Anmerkung: Die Referenz erhält dieselbe Berechtigung wie die Rechnung. Die Gruppe Accounting erhält lesenden Zugriff auf alle Konten 

    Beschreibung der Umsetzung / Notwendige Änderungen / Bewertung der Änderungen:
        -	Maskenenstellung „Konto“ + Definition Übersetzungsvariablen (Kontonummer, Bezeichnung und Ausprägung) 
        -	Installation + Einrichtung EVIATEC Content Manager (Prod + Test)
        -	Konfiguration CoMa (Invoice Ablage – Ablage der Rechnung unter Kreditor, zusätzlich Referenzierung unter Konto, falls Sachkonto noch nicht vorhanden Fehlermeldung „Sachkonto nicht vorhanden, bitte um Anlage“, falls Datum noch nicht vorhanden, Erstellung des Datums, Bestehende Ordnerstruktur) 
        -	Erstellung AS-Rule, welche die passende Maske für die Konten hinterlegt 
        -	Erstellung Regex, mit welchem die Informationen Kontonummer, Bezeichnung + Ausprägung aus der bisherigen Kurzbezeichnung extrahiert und im neuen Objekt hinterlegt werden können
        -	Definition EOM Input Verzeichnis für Import der Übersichtstabelle der Konten 
        -	Ummodelierung der Tabelle mit der aktuellen Übersicht der Konten für DB-Import, Anlegen in der Datenbank + Bereitstellung gewünschtes Tabellenformat an Kunde 
        -	Erstellung Stichwortliste der vorhandenen Konten +  hinterlegen im Formular 
        -	Setzen Feld Sachkonto als Pflichtfeld 
        -	Implementierung Buttons „Anlage Konto“ + „Anlage neues Geschäftsjahr

");

    chat_req = chat_req.append_message(ChatMessage::user(question));

    let chat_res = client
        .exec_chat_stream(MODEL, chat_req.clone(), None)
        .await?;

    let assistant_answer = print_chat_stream(chat_res, None).await?;

    // chat_req = chat_req.append_message(ChatMessage::assistant(assistant_answer.clone()));

    Ok(assistant_answer)
}
