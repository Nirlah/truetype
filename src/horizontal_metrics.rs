//! The [horizontal metrics][1].
//!
//! [1]: https://www.microsoft.com/typography/otspec/hmtx.htm

use {HorizontalHeader, MaximumProfile, Result, Tape, Walue};

table! {
    @define
    #[doc = "Horizontal metrics."]
    pub HorizontalMetrics {
        records            (Vec<Record>), // hMetrics
        left_side_bearings (Vec<i16>   ), // leftSideBearing
    }
}

table! {
    #[doc = "A record of horizontal metrics."]
    #[derive(Copy)]
    pub Record { // longHorMetric
        advance_width     (u16), // advanceWidth
        left_side_bearing (i16), // lsb
    }
}

impl HorizontalMetrics {
    /// Return the advance width and left side bearing.
    pub fn get(&self, mut index: usize) -> (u16, i16) {
        let longs = self.records.len();
        if index < longs {
            (
                self.records[index].advance_width,
                self.records[index].left_side_bearing,
            )
        } else {
            let shorts = self.left_side_bearings.len();
            index -= longs;
            if index < shorts {
                (
                    self.records[longs - 1].advance_width,
                    self.left_side_bearings[index],
                )
            } else {
                (
                    self.records[longs - 1].advance_width,
                    self.left_side_bearings[shorts - 1],
                )
            }
        }
    }
}

impl<'l> Walue<'l> for HorizontalMetrics {
    type Parameter = (&'l HorizontalHeader, &'l MaximumProfile);

    fn read<T: Tape>(tape: &mut T, (header, profile): Self::Parameter) -> Result<Self> {
        let metric_count = header.horizontal_metric_count as usize;
        let glyph_count = profile.glyph_count();
        if metric_count == 0 || metric_count > glyph_count {
            raise!("found a malformed horizontal header");
        }
        let bearing_count = glyph_count - metric_count;
        let mut table = HorizontalMetrics {
            records: Vec::with_capacity(metric_count),
            left_side_bearings: Vec::with_capacity(bearing_count),
        };
        for _ in 0..metric_count {
            table.records.push(tape.take()?);
        }
        for _ in 0..bearing_count {
            table.left_side_bearings.push(tape.take()?);
        }
        Ok(table)
    }
}
