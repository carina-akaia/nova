import dotenv_gleam
import gleam/bytes_builder
import gleam/erlang/os
import gleam/erlang/process.{type ExitReason}
import gleam/http.{Post}
import gleam/http/request
import gleam/http/response.{type Response}
import gleam/io
import gleam/json.{decode_bits, float, object, preprocessed_array, string}
import gleam/option.{type Option, Some}
import httpp/hackney
import httpp/streaming.{type Message, StreamingRequestHandler}

pub type MessageVariant {
  OpenaiMessage(role: String, content: String)
}

pub fn request_body(message: MessageVariant) {
  object([
    #("model", string("gpt-4o")),
    #("temperature", float(0.7)),
    #(
      "messages",
      preprocessed_array([
        object([
          #("role", string("system")),
          #("content", string("You are an assistant.")),
        ]),
        object([
          #("role", string(message.role)),
          #("content", string(message.content)),
        ]),
      ]),
    ),
  ])
  |> json.to_string
}

pub type State =
  String

fn on_data(
  msg: Message,
  _resp: Response(Nil),
  state: State,
) -> Result(State, ExitReason) {
  io.println("Message")
  io.debug(msg)
  io.println("State")
  io.debug(state)

  "Success!" |> Ok
}

pub fn on_message(
  msg_type: Message,
  _resp: Response(Nil),
  state: State,
) -> Result(State, ExitReason) {
  io.debug(msg_type)
  io.debug(state)
  io.debug("📨📨📨")

  state |> Ok
}

pub fn on_error(
  error: hackney.Error,
  resp: Option(Response(Nil)),
  state: State,
) -> Result(State, ExitReason) {
  io.print_error("💀💀💀")
  io.debug(error)

  case resp {
    Some(resp) -> {
      io.debug(resp)
      Nil
    }

    _ -> Nil
  }

  state |> Ok
}

pub fn send(endpoint_url: String, api_key: String, prompt: String) {
  let assert Ok(chat_request) = request.to(endpoint_url)

  let response_subject =
    chat_request
    |> request.prepend_header("Authorization", "Bearer " <> api_key)
    |> request.prepend_header("Content-Type", "application/json")
    |> request.set_method(Post)
    |> request.set_body(request_body(OpenaiMessage("user", prompt)))
    |> request.map(bytes_builder.from_string)

  let handler =
    StreamingRequestHandler(
      initial_state: "Initialized...",
      req: response_subject,
      on_data: on_data,
      on_message: on_message,
      on_error: on_error,
      initial_response_timeout: 100_000,
    )

  streaming.start(handler)
}

pub fn main() {
  dotenv_gleam.config_with("../../../../.env")

  let assert Ok(openai_api_key) = os.get_env("OPENAI_API_KEY")

  let reply =
    send(
      "https://api.openai.com/v1/chat/completions",
      openai_api_key,
      "Send back the following: 🦊",
    )

  let result = case reply {
    Ok(reply) -> process.receive(reply, 5000)
    Error(_error) -> Error(Nil)
  }

  io.debug(result)
}
