//! Minimal but standards-conformant STEP (ISO 10303-21) export.
//!
//! The previous implementation emitted bare `CARTESIAN_POINT` entities with a
//! literal `.` appended after Rust's `f64` Display output, producing text like
//! `CARTESIAN_POINT('',(10.1.,0.,0.))`. The trailing dot is not valid EXPRESS
//! real syntax, so every file this library produced was rejected by conformant
//! readers. The points were also orphaned: nothing referenced them, so even a
//! lenient reader would render an empty model.
//!
//! This module writes an AP214 `GEOMETRIC_CURVE_SET` containing one `POLYLINE`
//! per input path, wrapped in the product/context entities the schema requires.

use std::fmt::Write as _;

use crate::common::{Point3D, SpirographError};

/// Formats an `f64` as an EXPRESS `REAL` literal.
///
/// EXPRESS requires a decimal point in every real literal. Rust's `Display`
/// prints `1.0` as `"1"`, so a point must be appended when absent - but only
/// when absent, which is what the previous unconditional `"{}."` got wrong.
fn format_real(value: f64) -> Result<String, SpirographError> {
    if !value.is_finite() {
        return Err(SpirographError::ExportError(format!(
            "cannot write non-finite coordinate {} to a STEP file",
            value
        )));
    }
    let mut s = format!("{}", value);
    if !s.contains('.') && !s.contains('e') && !s.contains('E') {
        s.push('.');
    }
    Ok(s)
}

/// Escapes a string for use inside a STEP single-quoted literal.
fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

/// Writes `polylines` to `filename` as an ISO-10303-21 geometric curve set.
///
/// Each inner `Vec` becomes one `POLYLINE`. Paths with fewer than two points
/// cannot form a polyline and are skipped.
pub fn write_step_polylines(
    filename: &str,
    description: &str,
    polylines: &[Vec<Point3D>],
) -> Result<(), SpirographError> {
    let drawable: Vec<&Vec<Point3D>> = polylines.iter().filter(|p| p.len() >= 2).collect();
    if drawable.is_empty() {
        return Err(SpirographError::ExportError(
            "No geometry to export. Ensure the pattern was generated and contains \
             at least one path of two or more points."
                .to_string(),
        ));
    }

    let short_name = std::path::Path::new(filename)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "pattern.stp".to_string());
    let timestamp = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();

    let mut out = String::new();
    out.push_str("ISO-10303-21;\n");
    out.push_str("HEADER;\n");
    let _ = writeln!(out, "FILE_DESCRIPTION(('{}'),'2;1');", escape(description));
    let _ = writeln!(
        out,
        "FILE_NAME('{}','{}',(''),(''),'turtles','turtles','');",
        escape(&short_name),
        timestamp
    );
    out.push_str("FILE_SCHEMA(('AUTOMOTIVE_DESIGN { 1 0 10303 214 1 1 1 1 }'));\n");
    out.push_str("ENDSEC;\n");
    out.push_str("DATA;\n");

    // Fixed preamble: application/product context and the unit-bearing
    // geometric representation context that the curve set is expressed in.
    out.push_str("#1=APPLICATION_CONTEXT('automotive design');\n");
    out.push_str(
        "#2=APPLICATION_PROTOCOL_DEFINITION('international standard',\
         'automotive_design',2000,#1);\n",
    );
    out.push_str("#3=PRODUCT_CONTEXT('',#1,'mechanical');\n");
    let _ = writeln!(
        out,
        "#4=PRODUCT('{}','{}','',(#3));",
        escape(description),
        escape(description)
    );
    out.push_str("#5=PRODUCT_DEFINITION_FORMATION('','',#4);\n");
    out.push_str("#6=PRODUCT_DEFINITION_CONTEXT('part definition',#1,'design');\n");
    out.push_str("#7=PRODUCT_DEFINITION('','',#5,#6);\n");
    out.push_str("#8=PRODUCT_DEFINITION_SHAPE('','',#7);\n");
    out.push_str("#9=(LENGTH_UNIT()NAMED_UNIT(*)SI_UNIT(.MILLI.,.METRE.));\n");
    out.push_str("#10=(NAMED_UNIT(*)PLANE_ANGLE_UNIT()SI_UNIT($,.RADIAN.));\n");
    out.push_str("#11=(NAMED_UNIT(*)SI_UNIT($,.STERADIAN.)SOLID_ANGLE_UNIT());\n");
    out.push_str(
        "#12=UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-07),#9,\
         'distance_accuracy_value','');\n",
    );
    out.push_str(
        "#13=(GEOMETRIC_REPRESENTATION_CONTEXT(3)\
         GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT((#12))\
         GLOBAL_UNIT_ASSIGNED_CONTEXT((#9,#10,#11))REPRESENTATION_CONTEXT('',''));\n",
    );
    out.push_str("#14=CARTESIAN_POINT('',(0.,0.,0.));\n");
    out.push_str("#15=DIRECTION('',(0.,0.,1.));\n");
    out.push_str("#16=DIRECTION('',(1.,0.,0.));\n");
    out.push_str("#17=AXIS2_PLACEMENT_3D('',#14,#15,#16);\n");

    const FIRST_FREE_ID: usize = 18;
    let mut next_id = FIRST_FREE_ID;
    let mut polyline_ids: Vec<usize> = Vec::with_capacity(drawable.len());

    for path in &drawable {
        let mut point_ids: Vec<usize> = Vec::with_capacity(path.len());
        for p in path.iter() {
            let _ = writeln!(
                out,
                "#{}=CARTESIAN_POINT('',({},{},{}));",
                next_id,
                format_real(p.x)?,
                format_real(p.y)?,
                format_real(p.z)?
            );
            point_ids.push(next_id);
            next_id += 1;
        }
        let refs: Vec<String> = point_ids.iter().map(|id| format!("#{}", id)).collect();
        let _ = writeln!(out, "#{}=POLYLINE('',({}));", next_id, refs.join(","));
        polyline_ids.push(next_id);
        next_id += 1;
    }

    let curve_refs: Vec<String> = polyline_ids.iter().map(|id| format!("#{}", id)).collect();
    let curve_set_id = next_id;
    let _ = writeln!(
        out,
        "#{}=GEOMETRIC_CURVE_SET('{}',({}));",
        curve_set_id,
        escape(description),
        curve_refs.join(",")
    );
    next_id += 1;

    let shape_rep_id = next_id;
    let _ = writeln!(
        out,
        "#{}=SHAPE_REPRESENTATION('',(#17,#{}),#13);",
        shape_rep_id, curve_set_id
    );
    next_id += 1;

    let _ = writeln!(
        out,
        "#{}=SHAPE_DEFINITION_REPRESENTATION(#8,#{});",
        next_id, shape_rep_id
    );

    out.push_str("ENDSEC;\n");
    out.push_str("END-ISO-10303-21;\n");

    std::fs::write(filename, out)
        .map_err(|e| SpirographError::ExportError(format!("Failed to write STEP file: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_real_appends_point_only_when_missing() {
        assert_eq!(format_real(1.0).unwrap(), "1.");
        assert_eq!(format_real(0.0).unwrap(), "0.");
        assert_eq!(format_real(-2.0).unwrap(), "-2.");
        // The historic bug: a value that already carries a decimal point must
        // not gain a second one.
        assert_eq!(format_real(10.1).unwrap(), "10.1");
        assert_eq!(
            format_real(0.9720807030212073).unwrap(),
            "0.9720807030212073"
        );
    }

    #[test]
    fn test_format_real_rejects_non_finite() {
        assert!(format_real(f64::NAN).is_err());
        assert!(format_real(f64::INFINITY).is_err());
        assert!(format_real(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn test_escape_doubles_quotes() {
        assert_eq!(escape("it's"), "it''s");
    }

    #[test]
    fn test_write_step_rejects_empty_geometry() {
        let dir = std::env::temp_dir().join("turtles_step_empty.stp");
        let path = dir.to_str().unwrap();
        assert!(write_step_polylines(path, "empty", &[]).is_err());
        // A single point cannot form a polyline.
        assert!(write_step_polylines(path, "empty", &[vec![Point3D::new(0.0, 0.0, 0.0)]]).is_err());
    }

    #[test]
    fn test_write_step_emits_valid_reals_and_topology() {
        let dir = std::env::temp_dir().join("turtles_step_valid.stp");
        let path = dir.to_str().unwrap();
        let poly = vec![vec![
            Point3D::new(10.1, 0.0, 0.0),
            Point3D::new(9.89, 1.93, 0.0),
            Point3D::new(8.27, 5.45, 0.0),
        ]];
        write_step_polylines(path, "test pattern", &poly).unwrap();
        let content = std::fs::read_to_string(path).unwrap();

        assert!(content.starts_with("ISO-10303-21;"));
        assert!(content.ends_with("END-ISO-10303-21;\n"));
        // A trailing '.' is *required* by EXPRESS ("0." is a valid real), but a
        // real must never carry two of them - that was the original bug, where
        // `format!("{}.", 10.1)` produced `10.1.`.
        for token in content.split(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-')) {
            assert!(
                token.matches('.').count() <= 1,
                "malformed real with doubled decimal point: {:?}",
                token
            );
        }
        assert!(!content.contains(".."));
        assert!(content.contains("10.1,"));
        // Points must be referenced by a polyline, not orphaned.
        assert!(content.contains("POLYLINE"));
        assert!(content.contains("GEOMETRIC_CURVE_SET"));
        assert!(content.contains("SHAPE_DEFINITION_REPRESENTATION"));
        // The real filename is used, not a hardcoded one.
        assert!(content.contains("turtles_step_valid.stp"));

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_write_step_rejects_nan_coordinates() {
        let dir = std::env::temp_dir().join("turtles_step_nan.stp");
        let path = dir.to_str().unwrap();
        let poly = vec![vec![
            Point3D::new(f64::NAN, 0.0, 0.0),
            Point3D::new(1.0, 1.0, 0.0),
        ]];
        assert!(write_step_polylines(path, "nan", &poly).is_err());
    }
}
