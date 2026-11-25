Feature: guard

  The guard every Verdant Studio operation passes its host requests through,
  in app/src/Studio/Guard.lex.tex. The certified verdant_studio command and the
  app run the same definition, so the app answers a Studio guard request,
  studio TAB guard TAB VALUE, with exactly the text the command refuses with.

  @VS-08 @build @rejects
  Scenario: The host-request guard accepts a byte exactly when its code is at most 127, so it refuses by name every request that is not ASCII text, through one definition that the CLI and the app both run
    Given the guard model in app/src/Studio/Guard.lex.tex
    When prism-code check has Lean's kernel prove Studio.Guard.guard_refuses_exactly_non_ascii
    Then a byte is accepted exactly when its code is at most 127
    And a request that is ASCII text is answered accepted
    And prism-code build replays the request café.jsonl, answered with the refusal that names it
