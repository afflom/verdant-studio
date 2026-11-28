Feature: self-hosted

  Verdant Studio made in Verdant Studio, in app/src/Studio/SelfHosted.lex.tex. The app answers studio TAB self-hosted TAB part TAB PART TAB PRODUCER.

  @VS-76 @build
  Scenario: Verdant Studio is implemented in Verdant Studio, so Studio.SelfHosted accepts a part of Verdant Studio's production made in its own project, verdant-studio
    Given the self-hosting model in app/src/Studio/SelfHosted.lex.tex
    When prism-code check has Lean's kernel prove Studio.SelfHosted.a_part_made_in_its_own_project_is_self_hosted
    Then the brand, made in verdant-studio, is self-hosted
    And prism-code build replays that request, answered with that answer

  @VS-77 @build @rejects
  Scenario: Every part of Verdant Studio's production is made in Verdant Studio, so Studio.SelfHosted refuses a part made anywhere else, naming the part and its producer
    Given the self-hosting model in app/src/Studio/SelfHosted.lex.tex
    When prism-code check has Lean's kernel prove Studio.SelfHosted.refuses_a_part_made_elsewhere
    Then the brand made in other-tool is answered with the self-hosting refusal
    And prism-code build replays that request, answered with that refusal
