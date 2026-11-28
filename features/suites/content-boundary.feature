Feature: content-boundary

  Verdant Studio apart from the content it manages, in app/src/Studio/ContentBoundary.lex.tex. The app answers studio TAB boundary TAB place TAB ITEM TAB KIND....

  @VS-72 @build
  Scenario: Studio.ContentBoundary places a feature in Verdant Studio, the system of interest in the sense of ISO/IEC/IEEE 42010:2022
    Given the content boundary model in app/src/Studio/ContentBoundary.lex.tex
    When prism-code check has Lean's kernel prove Studio.ContentBoundary.places_a_feature_in_verdant_studio
    Then prism generation, a feature, is placed in Verdant Studio
    And prism-code build replays that request, answered with that answer

  @VS-73 @build
  Scenario: Studio.ContentBoundary places a demo, example, pipeline or project in Verdant Studio's environment, as content it manages and no feature of it (ISO/IEC/IEEE 42010:2022)
    Given the content boundary model in app/src/Studio/ContentBoundary.lex.tex
    When prism-code check has Lean's kernel prove Studio.ContentBoundary.places_managed_content_outside_verdant_studio
    Then starter game, a demo, is placed outside Verdant Studio
    And prism-code build replays that request, answered with that answer

  @VS-74 @build @rejects
  Scenario: Verdant Studio is differentiated from its managed content, so Studio.ContentBoundary refuses an item named both a feature and content, naming it
    Given the content boundary model in app/src/Studio/ContentBoundary.lex.tex
    When prism-code check has Lean's kernel prove Studio.ContentBoundary.refuses_an_item_named_both_feature_and_content
    Then prism generation named feature and pipeline is answered with the conflation refusal
    And prism-code build replays that request, answered with that refusal

  @VS-75 @build @rejects
  Scenario: Studio.ContentBoundary refuses a kind that is neither feature nor demo, example, pipeline or project, naming the kind and the item
    Given the content boundary model in app/src/Studio/ContentBoundary.lex.tex
    When prism-code check has Lean's kernel prove Studio.ContentBoundary.refuses_an_unknown_kind
    Then an item of kind widget is answered with the kind refusal
    And prism-code build replays that request, answered with that refusal
