Feature: relay

  Delivery of MCP messages over Stitch's sealed relay, in app/src/Studio/Agent/Interface/Relay.lex.tex. The app answers
  studio TAB relay TAB deliver TAB FRAMES TAB RECEIVED TAB BYTES TAB RECEIVED TAB DIGEST TAB UTF8 TAB ENVELOPE TAB SESSION TAB CONSENT TAB CALL.

  @VS-242 @build
  Scenario: The agent's MCP messages reach Studio over Stitch's sealed relay as a custom MCP transport (VS-100), so Studio.Agent.Interface.Relay delivers an assembled message whose digest, UTF-8, envelope, session and consent are verified (Stitch.Relay ready and deliveryAllowed, STITCH-RELAY-68A9222) to Studio.Agent.Interface.Transport, and answers what it decides as an MCP CallToolResult (MCP-SCHEMA-2026-07-28)
    Given the relay model in app/src/Studio/Agent/Interface/Relay.lex.tex
    When prism-code check has Lean's kernel prove Studio.Agent.Interface.Relay.delivers_an_assembled_verified_call_as_a_call_tool_result
    Then the admitted call is answered as a CallToolResult that is no error
    And prism-code build replays that request, answered with that answer

  @VS-243 @build
  Scenario: Errors that originate from a tool are reported inside the CallToolResult with isError true (MCP-SCHEMA-2026-07-28 CallToolResult), so Studio.Agent.Interface.Relay answers a call Transport refuses as an error result carrying the refusal
    Given the relay model in app/src/Studio/Agent/Interface/Relay.lex.tex
    When prism-code check has Lean's kernel prove Studio.Agent.Interface.Relay.a_refused_call_is_an_error_result
    Then the refused call is answered as a CallToolResult with isError true
    And prism-code build replays that request, answered with that answer

  @VS-244 @build @rejects
  Scenario: An unverified message cannot deliver (Stitch.Relay unverifiedMessageCannotDeliver and withdrawnConsentCannotPublish), so Studio.Agent.Interface.Relay holds a message any of whose checks is not verified, naming each one
    Given the relay model in app/src/Studio/Agent/Interface/Relay.lex.tex
    When prism-code check has Lean's kernel prove Studio.Agent.Interface.Relay.holds_an_unverified_message
    Then the message is held, naming the digest and the consent
    And prism-code build replays that request, answered with that answer

  @VS-245 @build @rejects
  Scenario: A message delivers only once every frame and byte of it is received (Stitch.Relay ready), so Studio.Agent.Interface.Relay holds a message still being assembled, naming what it has received
    Given the relay model in app/src/Studio/Agent/Interface/Relay.lex.tex
    When prism-code check has Lean's kernel prove Studio.Agent.Interface.Relay.holds_an_incomplete_message
    Then the message is held, naming the frames and bytes received
    And prism-code build replays that request, answered with that answer

  @VS-246 @build
  Scenario: A CallToolResult's text is a JSON string (MCP-SCHEMA-2026-07-28 TextContent, RFC 8259 section 7), so Studio.Agent.Interface.Relay escapes the quotation mark, the reverse solidus and every control character of the answer it carries
    Given the relay model in app/src/Studio/Agent/Interface/Relay.lex.tex
    When prism-code check has Lean's kernel prove Studio.Agent.Interface.Relay.escapes_a_json_string
    Then a quotation mark and a line feed are escaped
