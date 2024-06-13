use super::error::FFIError;

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum ScoringFunction {
    LrManhattanSc,
    LrManhattanCc,
    LrManhattanGn,
    LrManhattanCr,
    LrManhattanSp,
    LrManhattanVd,
    LrEuclideanCc,
    LrEuclideanSc,
    LrEuclideanGn,
    LrEuclideanCr,
    LrEuclideanSp,
    LrEuclideanVd,
    DtManhattanCc,
    DtManhattanSc,
    DtManhattanGn,
    DtManhattanCr,
    DtManhattanSp,
    DtManhattanVd,
    DtEuclideanCc,
    DtEuclideanSc,
    DtEuclideanGn,
    DtEuclideanCr,
    DtEuclideanSp,
    DtEuclideanVd,
}

pub fn enum_to_string(scoring_function: &ScoringFunction) -> Result<String, FFIError> {
    match scoring_function {
        ScoringFunction::LrManhattanSc => Ok("lr_manhattan_sc".to_string()),
        ScoringFunction::LrManhattanCc => Ok("lr_manhattan_cc".to_string()),
        ScoringFunction::LrManhattanGn => Ok("lr_manhattan_gn".to_string()),
        ScoringFunction::LrManhattanCr => Ok("lr_manhattan_cr".to_string()),
        ScoringFunction::LrManhattanSp => Ok("lr_manhattan_sp".to_string()),
        ScoringFunction::LrManhattanVd => Ok("lr_manhattan_vd".to_string()),
        ScoringFunction::LrEuclideanCc => Ok("lr_euclidean_cc".to_string()),
        ScoringFunction::LrEuclideanSc => Ok("lr_euclidean_sc".to_string()),
        ScoringFunction::LrEuclideanGn => Ok("lr_euclidean_gn".to_string()),
        ScoringFunction::LrEuclideanCr => Ok("lr_euclidean_cr".to_string()),
        ScoringFunction::LrEuclideanSp => Ok("lr_euclidean_sp".to_string()),
        ScoringFunction::LrEuclideanVd => Ok("lr_euclidean_vd".to_string()),
        ScoringFunction::DtManhattanCc => Ok("dt_manhattan_cc".to_string()),
        ScoringFunction::DtManhattanSc => Ok("dt_manhattan_sc".to_string()),
        ScoringFunction::DtManhattanGn => Ok("dt_manhattan_gn".to_string()),
        ScoringFunction::DtManhattanCr => Ok("dt_manhattan_cr".to_string()),
        ScoringFunction::DtManhattanSp => Ok("dt_manhattan_sp".to_string()),
        ScoringFunction::DtManhattanVd => Ok("dt_manhattan_vd".to_string()),
        ScoringFunction::DtEuclideanCc => Ok("dt_euclidean_cc".to_string()),
        ScoringFunction::DtEuclideanSc => Ok("dt_euclidean_sc".to_string()),
        ScoringFunction::DtEuclideanGn => Ok("dt_euclidean_gn".to_string()),
        ScoringFunction::DtEuclideanCr => Ok("dt_euclidean_cr".to_string()),
        ScoringFunction::DtEuclideanSp => Ok("dt_euclidean_sp".to_string()),
        ScoringFunction::DtEuclideanVd => Ok("dt_euclidean_vd".to_string()),
        _ => Err(FFIError::ScoringFunctionNotFound),
    }
}

pub fn enum_to_function(
    scoring_function: &ScoringFunction,
) -> Result<abd_clam::graph::MetaMLScorer, FFIError> {
    let pretrained_models = abd_clam::chaoda::pretrained_models::get_meta_ml_scorers();
    match enum_to_string(scoring_function) {
        Ok(function_name) => {
            match pretrained_models
                .into_iter()
                .find(|item| item.0 == function_name)
            {
                Some(result) => Ok(result.1),
                None => Err(FFIError::ScoringFunctionNotFound),
            }
        }
        Err(e) => Err(e),
    }
}
