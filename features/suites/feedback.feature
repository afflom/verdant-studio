Feature: feedback

  In-game player feedback (REQ-PLAYER-FEEDBACK), in app/src/Studio/Feedback.lex.tex
  and app/src/Studio/Feedback/. Players never sign in to Studio: the Prism they
  play sends its feedback forms, and the app answers feedback TAB QUORUM, then
  TAB each feature the creator assessed for immediate action (ISO 10002), then
  one form per line, with the catalog: each incident or complaint as its own
  single line, and each feature's feedback as one consensus or pending line
  with its distinct players, prompted players and earliest received time. It
  answers feedback-lead-time TAB RECEIVED TAB PUBLISHED with the lead time.

  @VS-21 @build @rejects
  Scenario: Player feedback arrives through the in-game feedback form, kind TAB channel TAB feature TAB received TAB player TAB text, of kind feedback or complaint (ISO 10002) or incident (ISO/IEC/IEEE 29119-3), received in decimal seconds; a form of any other shape is refused by name
    Given the feedback model in app/src/Studio/Feedback/Intake.lex.tex
    When prism-code check has Lean's kernel prove Studio.Feedback.Intake.reads_a_decimal, Studio.Feedback.Intake.writes_a_decimal and Studio.Feedback.refuses_an_unknown_kind_or_channel
    Then the decimal 1764200000 is read and written back, the empty text and 1a are no decimal, and a form of kind rate or channel email is answered with the form refusal
    And prism-code build replays the request feedback TAB 2 LF rate TAB form TAB jump TAB 1 TAB p1 TAB, answered with that refusal

  @VS-22 @build
  Scenario: A form is either the player's own in-game form or the answer to a creator's prompt for feedback on a feature, and the catalog counts the players who answered a prompt
    Given the feedback model in app/src/Studio/Feedback/Solicitation.lex.tex
    When prism-code check has Lean's kernel prove Studio.Feedback.Solicitation.knows_exactly_the_form_and_the_prompt and Studio.Feedback.consensus_counts_distinct_players
    Then the channels form and prompt are known and email is not, and a feature that player p1 answered through a prompt counts 1 prompted player

  @VS-23 @build
  Scenario: Feedback is masked: the catalog never carries the player's identifier, and every mention of it in an incident's or complaint's text is masked (ISO/IEC 29100 anonymization)
    Given the feedback model in app/src/Studio/Feedback/Masking.lex.tex
    When prism-code check has Lean's kernel prove Studio.Feedback.Masking.masks_every_mention_of_the_player, Studio.Feedback.Masking.keeps_a_text_without_the_player and Studio.Feedback.an_incident_acts_alone_and_masked
    Then p7 crashed when p7 jumped is masked as * crashed when * jumped, and the incident line for player p7 carries * crashed here and no p7
    And prism-code build replays that incident, answered with the masked single line

  @VS-24 @build @rejects
  Scenario: Feedback on a feature is distilled by consensus: it reaches consensus when at least the quorum of distinct players gave it, one player twice stays pending, and a quorum below 2 is refused as a single interest
    Given the feedback model in app/src/Studio/Feedback.lex.tex
    When prism-code check has Lean's kernel prove Studio.Feedback.consensus_counts_distinct_players, Studio.Feedback.one_player_twice_stays_pending and Studio.Feedback.one_player_is_no_consensus
    Then p1 by form, p1 by prompt and p2 on the feature jump reach consensus at quorum 2 as 2 players, p1 twice stays pending as 1 player, and a quorum of 0 or 1 is answered with the quorum refusal
    And prism-code build replays the request feedback TAB 1, answered with that refusal

  @VS-25 @build
  Scenario: An incident or a complaint is actioned on its own, without consensus, as a single entry carrying its masked text
    Given the feedback model in app/src/Studio/Feedback/Incident.lex.tex
    When prism-code check has Lean's kernel prove Studio.Feedback.Incident.an_incident_or_a_complaint_acts_alone and Studio.Feedback.an_incident_acts_alone_and_masked
    Then an incident and a complaint act alone and feedback does not, and an incident is emitted as its own single line ahead of the pending feedback on the same feature

  @VS-26 @build @rejects
  Scenario: The feedback-to-publish lead time (DORA change lead time) is the published time less the received time, and a release published before the feedback it answers is refused
    Given the feedback model in app/src/Studio/Feedback/LeadTime.lex.tex
    When prism-code check has Lean's kernel prove Studio.Feedback.LeadTime.lead_time_is_published_less_received and Studio.Feedback.LeadTime.refuses_a_release_before_its_feedback
    Then for every received time and every duration took, a release published took seconds after its feedback has lead time took, and a release at 9 for feedback received at 10 is answered with the lead-time refusal
    And prism-code build replays feedback-lead-time TAB 100 TAB 3700, answered 3600, and feedback-lead-time TAB 10 TAB 9, answered with that refusal

  @VS-32 @build
  Scenario: A single request on a feature that the creator's initial assessment (ISO 10002) marks for immediate action, sent as feedback TAB QUORUM TAB FEATURE, is actioned on its own without consensus; a player cannot assess their own form, which is refused if it tries
    Given the feedback model in app/src/Studio/Feedback/Assessment.lex.tex
    When prism-code check has Lean's kernel prove Studio.Feedback.Assessment.acts_on_only_the_features_the_creator_assessed and Studio.Feedback.a_request_the_creator_assessed_acts_alone
    Then feedback on a feature the creator assessed is its own single line with its player masked, the same form without that assessment stays pending, and a form that carries its own assessment is answered with the form refusal
    And prism-code build replays the assessed request, answered with its single line, and the self-assessed form, answered with the form refusal
