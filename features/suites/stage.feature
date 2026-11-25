Feature: stage

  The line checks of stage-bindings, in app/src/Studio/Stage.lex.tex. The
  certified verdant_studio command reads each binding line and decides it with
  these definitions, and the app decides a Studio stage request, studio TAB
  stage-bindings TAB ID TAB SCHEMA TAB REDISTRIBUTION TAB URL, with the same
  ones, so both answer with exactly the same text.

  @VS-09 @build @rejects
  Scenario: stage-bindings refuses a line that is not one prismpm/authority-binding/1 binding with an id, and accepts one that is, through one definition that the CLI and the app both run
    Given the stage model in app/src/Studio/Stage.lex.tex
    When prism-code check has Lean's kernel prove Studio.Stage.stage_refuses_a_line_that_is_no_binding and Studio.Stage.stage_accepts_a_named_binding
    Then a line that is no binding is answered with the binding refusal, whatever its source
    And prism-code build replays the W3C-SOSA-2017 binding, answered accepted
    And prism-code build replays that binding without its schema, answered with the binding refusal
    And prism-code build replays a stage request with only an id, answered invalid request

  @VS-10 @build @rejects
  Scenario: stage-bindings refuses a redistributable binding whose source.url ends in /, naming the binding, because ingest-authority saves the download under the last segment of the URL's path
    Given the stage model in app/src/Studio/Stage.lex.tex
    When prism-code check has Lean's kernel prove Studio.Stage.stage_refuses_an_unnamed_artifact
    Then a binding whose download has no file name is answered with the refusal that names it
    And prism-code build replays the W3C-XML-1.0-5E binding with a source.url ending in /, answered with that refusal
