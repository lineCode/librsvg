extern crate cairo;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Segment {
    Degenerate {            // A single lone point
        x: f64,
        y: f64
    },

    LineOrCurve {
        x1: f64, y1: f64,
        x2: f64, y2: f64,
        x3: f64, y3: f64,
        x4: f64, y4: f64
    },
}

enum SegmentState {
    Start,
    End
}

/* This converts a cairo_path_t into a list of curveto-like segments.  Each segment can be:
 *
 * 1. Segment::Degenerate => the segment is actually a single point (x, y)
 *
 * 2. Segment::LineOrCurve => either a lineto or a curveto (or the effective lineto that results from a closepath).
 *    We have the following points:
 *       P1 = (x1, y1)
 *       P2 = (x2, y2)
 *       P3 = (x3, y3)
 *       P4 = (x4, y4)
 *
 *    The start and end points are P1 and P4, respectively.
 *    The tangent at the start point is given by the vector (P2 - P1).
 *    The tangent at the end point is given by the vector (P4 - P3).
 *    The tangents also work if the segment refers to a lineto (they will both just point in the same direction).
 */

const EPSILON: f64 = 1e-10;

fn double_equals (a: f64, b: f64) -> bool {
    (a - b).abs () < EPSILON
}

pub fn path_to_segments (path: cairo::Path) -> Vec<Segment> {
    let mut last_x: f64;
    let mut last_y: f64;
    let mut cur_x: f64;
    let mut cur_y: f64;
    let mut subpath_start_x: f64;
    let mut subpath_start_y: f64;
    let mut segments: Vec<Segment>;
    let mut state: SegmentState;

    cur_x = 0.0;
    cur_y = 0.0;
    subpath_start_x = 0.0;
    subpath_start_y = 0.0;

    segments = Vec::new ();
    state = SegmentState::End;

    for cairo_segment in path.iter () {
        last_x = cur_x;
        last_y = cur_y;

        let needs_new_segment: bool;
        let seg: Segment;

        match cairo_segment {
            cairo::PathSegment::MoveTo ((x, y)) => {
                cur_x = x;
                cur_y = y;

                subpath_start_x = cur_x;
                subpath_start_y = cur_y;

                seg = Segment::Degenerate {
                    x: cur_x,
                    y: cur_y
                };
                needs_new_segment = true;

                state = SegmentState::Start;
            },

            cairo::PathSegment::LineTo ((x, y)) => {
                cur_x = x;
                cur_y = y;

                match state {
                    SegmentState::Start => {
                        state = SegmentState::End;
                        needs_new_segment = false;
                    },

                    SegmentState::End => {
                        needs_new_segment = true;
                    }
                }

                seg = Segment::LineOrCurve {
                    x1: last_x,
                    y1: last_y,

                    x2: cur_x,
                    y2: cur_y,

                    x3: last_x,
                    y3: last_y,

                    x4: cur_x,
                    y4: cur_y,
                };
            },

            cairo::PathSegment::CurveTo ((mut x2, mut y2), (mut x3, mut y3), (x4, y4)) => {
                cur_x = x4;
                cur_y = y4;

                let x1 = last_x;
                let y1 = last_y;

                match state {
                    SegmentState::Start => {
                        state = SegmentState::End;
                        needs_new_segment = false;
                    },

                    SegmentState::End => {
                        needs_new_segment = true;
                    }
                }

                /* Fix the tangents for when the middle control points coincide with their respective endpoints */

                if double_equals (x2, x1) && double_equals (y2, y1) {
                    x2 = x3;
                    y2 = y3;
                }

                if double_equals (x3, x4) && double_equals (y3, y4) {
                    x3 = x2;
                    y3 = y2;
                }

                seg = Segment::LineOrCurve {
                    x1: x1,
                    y1: y1,

                    x2: x2,
                    y2: y2,

                    x3: x3,
                    y3: y3,

                    x4: x4,
                    y4: y4,
                };
            }

            cairo::PathSegment::ClosePath => {
                cur_x = subpath_start_x;
                cur_y = subpath_start_y;

                match state {
                    SegmentState::Start => {
                        state = SegmentState::End;
                        needs_new_segment = false;
                    },

                    SegmentState::End => {
                        needs_new_segment = false;
                        /* nothing; closepath after moveto (or a single lone closepath) does nothing */
                    }
                }

                seg = Segment::LineOrCurve {
                    x1: last_x,
                    y1: last_y,

                    x2: cur_x,
                    y2: cur_y,

                    x3: last_x,
                    y3: last_y,

                    x4: cur_x,
                    y4: cur_y,
                };
            }
        }

        if needs_new_segment {
            segments.push (seg);
        } else {
            let len = segments.len ();
            segments[len - 1] = seg;
        }
    }

    segments
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate cairo;

    fn setup_open_path () -> cairo::Path {
        let surf = cairo::ImageSurface::create (cairo::Format::Rgb24, 1, 1);
        let cr = cairo::Context::new (&surf);

        cr.move_to (10.0, 10.0);
        cr.line_to (20.0, 10.0);
        cr.line_to (20.0, 20.0);

        let path = cr.copy_path ();
        path
    }

    #[test]
    fn path_to_segments_handles_open_path () {
        let expected_segments: Vec<Segment> = vec![
            Segment::LineOrCurve { x1: 10.0, y1: 10.0, x2: 20.0, y2: 10.0, x3: 10.0, y3: 10.0, x4: 20.0, y4: 10.0 },
            Segment::LineOrCurve { x1: 20.0, y1: 10.0, x2: 20.0, y2: 20.0, x3: 20.0, y3: 10.0, x4: 20.0, y4: 20.0 }
        ];

        let path = setup_open_path ();
        let segments = path_to_segments (path);

        assert_eq! (expected_segments, segments);
    }
}
