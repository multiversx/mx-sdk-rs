use super::report_feature::ReportFeature;

pub struct ReportCreator {
    pub report_features: Vec<ReportFeature>,
    pub skip_build: bool,
    pub skip_twiggy: bool,
    pub require_twiggy_paths: bool,
    pub build_options: String,
    pub build_args: Vec<String>,
}

impl ReportCreator {
}