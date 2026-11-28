Feature: typeface

  The typeface generator Studio.Generator.Typeface, in
  app/src/Studio/Generator/Typeface/: a Prism's font/otf facet, whose glyphs
  are Computer Modern's METAFONT programs run with METAFONT's own arithmetic
  and written as CFF outlines.

  @VS-81 @build
  Scenario: The typeface generator's arithmetic is METAFONT's, as mf.web 2.71828182 computes it: make_fraction, take_fraction, make_scaled, take_scaled, square_rt, pyth_add, n_arg and n_sin_cos give mf.web's results bit for bit, rounding and overflow to el_gordo included
    Given the arithmetic model in app/src/Studio/Generator/Typeface/Scaled.lex.tex
    When prism-code check has Lean's kernel prove Studio.Generator.Typeface.Scaled.makes_fractions_as_mf_web_does and Studio.Generator.Typeface.Scaled.takes_fractions_as_mf_web_does and Studio.Generator.Typeface.Scaled.makes_scaled_as_mf_web_does and Studio.Generator.Typeface.Scaled.takes_scaled_as_mf_web_does and Studio.Generator.Typeface.Scaled.roots_as_mf_web_does and Studio.Generator.Typeface.Scaled.measures_angles_as_mf_web_does and Studio.Generator.Typeface.Scaled.adds_pythagorean_as_mf_web_does and Studio.Generator.Typeface.Scaled.sines_and_cosines_as_mf_web_does
    Then each routine answers the value mf.web's own routine computes for the same integers

  @VS-87 @build
  Scenario: The typeface generator chooses a path's control points as METAFONT does: make_choices of mf.web 2.71828182 (Hobby's mock-curvature equations with tension, curl, given directions, explicit controls, tension atleast and cycles) gives mf.web's control points bit for bit
    Given the path model in app/src/Studio/Generator/Typeface/Choices.lex.tex
    When prism-code check has Lean's kernel prove Studio.Generator.Typeface.Choices.chooses_a_curled_arc_as_metafont_does and Studio.Generator.Typeface.Choices.chooses_a_cycle_as_metafont_does and Studio.Generator.Typeface.Choices.chooses_given_directions_and_tensions_as_metafont_does and Studio.Generator.Typeface.Choices.chooses_beside_explicit_controls_as_metafont_does and Studio.Generator.Typeface.Choices.chooses_tension_at_least_as_metafont_does and Studio.Generator.Typeface.Choices.joins_equal_knots_as_metafont_does and Studio.Generator.Typeface.Choices.chooses_two_given_directions_as_metafont_does and Studio.Generator.Typeface.Choices.draws_a_curled_straight_line_as_metafont_does
    Then each path gets the control points mf.web's make_choices computes for the same knots

  @VS-91 @build
  Scenario: The typeface generator makes pencircle pens as METAFONT does: make_ellipse of mf.web 2.71828182, with the axes and angle that pencircle transformed by a transform becomes and with fillin, gives mf.web's pen polygon vertex for vertex
    Given the pen model in app/src/Studio/Generator/Typeface/Pen.lex.tex
    When prism-code check has Lean's kernel prove Studio.Generator.Typeface.Pen.makes_a_pencircle_of_diameter_three_as_metafont_does and Studio.Generator.Typeface.Pen.makes_a_flat_pen_as_metafont_does and Studio.Generator.Typeface.Pen.makes_a_quarter_turned_pen_as_metafont_does and Studio.Generator.Typeface.Pen.makes_a_slanted_pen_as_metafont_does and Studio.Generator.Typeface.Pen.makes_a_filled_in_pen_as_metafont_does and Studio.Generator.Typeface.Pen.follows_mf_web_where_tex_live_2023_differs
    Then each transform gets the pen polygon mf.web's make_ellipse computes for the same pencircle

  @VS-92 @build
  Scenario: The typeface generator cuts a cyclic path into octants as METAFONT does: make_spec of mf.web 2.71828182 (quadrant and octant subdivision, dead cubics, octant boundaries) gives mf.web's cycle spec knot for knot and its turning number
    Given the cycle spec model in app/src/Studio/Generator/Typeface/Spec.lex.tex
    When prism-code check has Lean's kernel prove Studio.Generator.Typeface.Spec.divides_a_circle_into_eight_octants_as_metafont_does and Studio.Generator.Typeface.Spec.divides_a_triangle_at_its_corners_as_metafont_does and Studio.Generator.Typeface.Spec.turns_a_reversal_counterclockwise_as_metafont_does and Studio.Generator.Typeface.Spec.turns_a_doubled_reversal_clockwise_as_metafont_does and Studio.Generator.Typeface.Spec.splits_an_inflected_curve_as_metafont_does
    Then each path gets the octant knots and the turning number mf.web's make_spec computes for the same path

  @VS-93 @build
  Scenario: The typeface generator strokes a cycle spec with a pen as METAFONT does: make_pen's offsets, offset_prep and fill_envelope of mf.web 2.71828182 give mf.web's envelope outline segment for segment
    Given the stroke envelope model in app/src/Studio/Generator/Typeface/Envelope.lex.tex
    When prism-code check has Lean's kernel prove Studio.Generator.Typeface.Envelope.strokes_a_circle_with_a_round_pen_as_metafont_does and Studio.Generator.Typeface.Envelope.strokes_a_triangle_with_a_flat_pen_as_metafont_does and Studio.Generator.Typeface.Envelope.strokes_an_inflected_curve_with_a_slanted_pen_as_metafont_does and Studio.Generator.Typeface.Envelope.strokes_a_line_there_and_back_with_a_round_pen_as_metafont_does
    Then each path and pen get the envelope lines and cubics mf.web's fill_envelope traces for the same stroke
