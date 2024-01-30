use abd_clam::MetaMLScorer;

// pub type MetaMLScorer = Box<fn(abd_clam::::Ratios) -> f64>;

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

pub fn enum_to_string(scoring_function: &ScoringFunction) -> String {
    match scoring_function {
        ScoringFunction::LrManhattanSc => "lr_manhattan_sc".to_string(),
        ScoringFunction::LrManhattanCc => "lr_manhattan_cc".to_string(),
        ScoringFunction::LrManhattanGn => "lr_manhattan_gn".to_string(),
        ScoringFunction::LrManhattanCr => "lr_manhattan_cr".to_string(),
        ScoringFunction::LrManhattanSp => "lr_manhattan_sp".to_string(),
        ScoringFunction::LrManhattanVd => "lr_manhattan_vd".to_string(),
        ScoringFunction::LrEuclideanCc => "lr_euclidean_cc".to_string(),
        ScoringFunction::LrEuclideanSc => "lr_euclidean_sc".to_string(),
        ScoringFunction::LrEuclideanGn => "lr_euclidean_gn".to_string(),
        ScoringFunction::LrEuclideanCr => "lr_euclidean_cr".to_string(),
        ScoringFunction::LrEuclideanSp => "lr_euclidean_sp".to_string(),
        ScoringFunction::LrEuclideanVd => "lr_euclidean_vd".to_string(),
        ScoringFunction::DtManhattanCc => "dt_manhattan_cc".to_string(),
        ScoringFunction::DtManhattanSc => "dt_manhattan_sc".to_string(),
        ScoringFunction::DtManhattanGn => "dt_manhattan_gn".to_string(),
        ScoringFunction::DtManhattanCr => "dt_manhattan_cr".to_string(),
        ScoringFunction::DtManhattanSp => "dt_manhattan_sp".to_string(),
        ScoringFunction::DtManhattanVd => "dt_manhattan_vd".to_string(),
        ScoringFunction::DtEuclideanCc => "dt_euclidean_cc".to_string(),
        ScoringFunction::DtEuclideanSc => "dt_euclidean_sc".to_string(),
        ScoringFunction::DtEuclideanGn => "dt_euclidean_gn".to_string(),
        ScoringFunction::DtEuclideanCr => "dt_euclidean_cr".to_string(),
        ScoringFunction::DtEuclideanSp => "dt_euclidean_sp".to_string(),
        ScoringFunction::DtEuclideanVd => "dt_euclidean_vd".to_string(),
    }
}

pub fn enum_to_function(scoring_function: &ScoringFunction) -> Option<MetaMLScorer> {
    let pretrained_models = abd_clam::chaoda::pretrained_models::get_meta_ml_scorers();
    let function_name = enum_to_string(scoring_function);

    match pretrained_models
        .into_iter()
        .find(|item| item.0 == function_name)
    {
        Some(result) => Some(result.1),
        None => None,
    }
}
