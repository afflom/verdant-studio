Feature: telemetry

  The OpenTelemetry signals of each operation of Verdant Studio, in app/src/Studio/Telemetry.lex.tex. The app answers
  studio TAB telemetry TAB TRACEPARENT TAB SPAN-ID TAB PRISM-ID TAB INSTANCE TAB START TAB END TAB OPERATION TAB OUTCOME with the operation's span, log record and metrics as OTLP/JSON lines, or a refusal.

  @VS-250 @build
  Scenario: Every operation of Verdant Studio is observable (REQ-OBSERVABILITY), so Studio.Telemetry answers an operation's W3C traceparent, span id, acting Prism ID, instance, Unix-nanosecond start and end, name and outcome with its OpenTelemetry signals as OTLP/JSON lines (OTEL-SPEC-1.60.0-FILE-EXPORTER): a SERVER span named RUN and the operation (OTEL-SEMCONV-1.37.0-CICD-SPANS), child of the traceparent's parent-id in its trace, an INFO log record correlated with that span (OTEL-SPEC-1.60.0-LOGS-DATA-MODEL), and a cicd.pipeline.run.duration histogram point in seconds (OTEL-SEMCONV-1.37.0-CICD-METRICS), each attributed to the Prism ID as user.id and to the instance as service.instance.id
    Given the telemetry model in app/src/Studio/Telemetry.lex.tex
    When prism-code check has Lean's kernel prove Studio.Telemetry.traces_an_operation_as_its_otlp_signals
    Then an observer operation by prism:agent that took 0.25 s is answered with its span, log record and duration metric lines
    And prism-code build replays that request, answered with that answer

  @VS-251 @build
  Scenario: Each refusal is the error from which REQ-SELF-HEALING takes its observation, so Studio.Telemetry records an operation whose outcome is a refusal with span status ERROR and the refusal as its message (OTEL-SPEC-1.60.0-TRACE-API), an ERROR log record (SeverityNumber 17) whose body is the refusal, cicd.pipeline.result failure with error.type _OTHER (OTEL-SEMCONV-1.37.0-ERROR), and a cicd.pipeline.run.errors counter point
    Given the telemetry model in app/src/Studio/Telemetry.lex.tex
    When prism-code check has Lean's kernel prove Studio.Telemetry.traces_a_refused_operation_as_an_error_span and Studio.Telemetry.traces_a_refused_operation_as_an_error_log and Studio.Telemetry.traces_a_refused_operation_as_an_error_metrics
    Then a refused observer operation is answered with an error span, an ERROR log record and the errors counter
    And prism-code build replays that request, answered with that answer

  @VS-252 @build @rejects
  Scenario: W3C Trace Context makes an all-zero trace-id or parent-id invalid (W3C-TRACE-CONTEXT-1), so Studio.Telemetry refuses a traceparent that has one, naming it
    Given the telemetry model in app/src/Studio/Telemetry.lex.tex
    When prism-code check has Lean's kernel prove Studio.Telemetry.refuses_an_all_zero_trace_id
    Then a traceparent with an all-zero trace-id is answered with the refusal
    And prism-code build replays that request, answered with that refusal

  @VS-253 @build @rejects
  Scenario: A traceparent is version-trace-id-parent-id-trace-flags in 2, 32, 16 and 2 lowercase hex digits, and version ff is invalid (W3C-TRACE-CONTEXT-1), so Studio.Telemetry refuses any other traceparent, naming it
    Given the telemetry model in app/src/Studio/Telemetry.lex.tex
    When prism-code check has Lean's kernel prove Studio.Telemetry.refuses_traceparent_version_ff
    Then a traceparent of version ff is answered with the refusal
    And prism-code build replays that request, answered with that refusal

  @VS-254 @build @rejects
  Scenario: A span id is 8 bytes in 16 lowercase hex digits and not all zero (W3C-TRACE-CONTEXT-1, OTEL-SPEC-1.60.0-TRACE-API), so Studio.Telemetry refuses any other span id, naming it
    Given the telemetry model in app/src/Studio/Telemetry.lex.tex
    When prism-code check has Lean's kernel prove Studio.Telemetry.refuses_an_all_zero_span_id
    Then an all-zero span id is answered with the refusal
    And prism-code build replays that request, answered with that refusal

  @VS-255 @build @rejects
  Scenario: Runtime authority is the Prism ID (REQ-SELF-RUNTIME), and the log records are the audit trail of who did what (ISO-IEC-27002-2022-PREVIEW 8.15), so Studio.Telemetry refuses an operation whose actor names no Prism ID
    Given the telemetry model in app/src/Studio/Telemetry.lex.tex
    When prism-code check has Lean's kernel prove Studio.Telemetry.refuses_an_actor_with_no_prism_id
    Then an operation by alex is answered with the refusal
    And prism-code build replays that request, answered with that refusal

  @VS-256 @build @rejects
  Scenario: A span's times are Unix nanoseconds and it ends no earlier than it starts (OTLP-1.11.1-TRACE-PROTO), so Studio.Telemetry refuses times that are not decimal or that end before they start, naming them
    Given the telemetry model in app/src/Studio/Telemetry.lex.tex
    When prism-code check has Lean's kernel prove Studio.Telemetry.refuses_a_span_that_ends_before_it_starts
    Then an end before the start is answered with the refusal
    And prism-code build replays that request, answered with that refusal

  @VS-257 @build
  Scenario: OTLP/JSON encodes every string as JSON (OTLP-1.11.1-SPECIFICATION), so Studio.Telemetry escapes each quotation mark, reverse solidus and control character of an operation's name, actor, instance and outcome
    Given the telemetry model in app/src/Studio/Telemetry.lex.tex
    When prism-code check has Lean's kernel prove Studio.Telemetry.escapes_every_string_as_json
    Then say "hi", a tab, now and a reverse solidus is written as one JSON string

  @VS-258 @open
  Scenario: Verdant Studio's own runtime records every operation it runs, on every instance, through Studio.Telemetry, appending its signals to the instance's OTLP JSON lines file, so each operation and each refusal is logged, measured and traced (REQ-OBSERVABILITY, REQ-SELF-RUNTIME); until the runtime host calls the telemetry route for each operation, the signals are answered on request only
    Given the telemetry route of VS-250
    When the runtime host runs any operation of the app
    Then it appends that operation's span, log record and metrics to its instance's OTLP JSON lines file
