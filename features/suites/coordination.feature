Feature: coordination

  Coordination between the maintainer and its linked agent, in app/src/Studio/Coordination.lex.tex. The app
  answers studio TAB coordination TAB send TAB SENDER TAB RECIPIENT TAB CHANNEL TAB AGENT=MAINTAINER... with the
  message carried on the Verdant Studio UI, or a refusal.

  @VS-65 @build
  Scenario: Studio.Coordination carries every message between the maintainer and its linked agent on the Verdant Studio UI, in either direction, sealed to their Prism IDs by Verdant Engine's Prism ID boundary privacy, VE-43
    Given the coordination model in app/src/Studio/Coordination.lex.tex
    When prism-code check has Lean's kernel prove Studio.Coordination.carries_a_message_on_the_studio_ui
    Then a message from prism:alex to its linked agent prism:agent on the Verdant Studio UI is carried
    And prism-code build replays that request, answered with that answer

  @VS-66 @build @rejects
  Scenario: All coordination between the maintainer and the agent happens through the Verdant Studio UI, so Studio.Coordination refuses a message between them on any other channel, naming it
    Given the coordination model in app/src/Studio/Coordination.lex.tex
    When prism-code check has Lean's kernel prove Studio.Coordination.refuses_a_message_off_the_studio_ui
    Then a message from prism:agent to prism:alex on github is answered with the channel refusal
    And prism-code build replays that request, answered with that refusal

  @VS-67 @build @rejects
  Scenario: Studio.Coordination carries messages only between a maintainer and its linked agent, so it refuses any other pair, naming both
    Given the coordination model in app/src/Studio/Coordination.lex.tex
    When prism-code check has Lean's kernel prove Studio.Coordination.refuses_a_message_between_unlinked_prism_ids
    Then a message from prism:bob, not linked to prism:alex, is answered with the pair refusal
    And prism-code build replays that request, answered with that refusal

  @VS-68 @open
  Scenario: The Verdant Studio UI carries the messages between the maintainer and the agent that Studio.Coordination admits, so every interaction between them happens there; until the UI serves that channel, they coordinate in the project chat and the GitHub mirror
    Given the message of VS-65
    When the Verdant Studio UI serves the coordination channel
    Then the maintainer and the agent exchange it there, and nowhere else
