use strum_macros::EnumIter;

#[derive(Debug, PartialEq, EnumIter, Clone)]
pub enum Roles {
    Conceptualization,
    DataCuration,
    FormalAnalysis,
    FundingAcquisition,
    Investigation,
    Methodology,
    ProjectAdministration,
    Resources,
    Software,
    Supervision,
    Validation,
    Visualization,
    WritingOriginalDraft,
    WritingReviewEditing,
}

impl Roles {
    pub fn to_string(&self) -> String {
        let value = match self {
            Roles::Conceptualization => "Conceptualization",
            Roles::DataCuration => "Data curation",
            Roles::FormalAnalysis => "Formal analysis",
            Roles::FundingAcquisition => "Funding acquisition",
            Roles::Investigation => "Investigation",
            Roles::Methodology => "Methodology",
            Roles::ProjectAdministration => "Project administration",
            Roles::Resources => "Resources",
            Roles::Software => "Software",
            Roles::Supervision => "Supervision",
            Roles::Validation => "Validation",
            Roles::Visualization => "Visualization",
            Roles::WritingOriginalDraft => "Writing - Original Draft",
            Roles::WritingReviewEditing => "Writing - Review & Editing",
        };

        value.to_string()
    }

    pub fn description(&self) -> String {
        let value = match self {
            Roles::Conceptualization => {
                "Ideas; formulation or evolution of overarching research goals and aims."
            }
            Roles::DataCuration => "Management activities to annotate (produce metadata), scrub data and maintain research data (including software code, where it is necessary for interpreting the data itself) for initial use and later re-use.",
            Roles::FormalAnalysis => "Application of statistical, mathematical, computational, or other formal techniques to analyze or synthesize study data.",
            Roles::FundingAcquisition => "Acquisition of the financial support for the project leading to this publication.",
            Roles::Investigation => "Conducting a research and investigation process, specifically performing the experiments, or data/evidence collection.",
            Roles::Methodology => "Development or design of methodology; creation of models.",
            Roles::ProjectAdministration => "Management and coordination responsibility for the research activity planning and execution.",
            Roles::Resources => "Provision of study materials, reagents, materials, patients, laboratory samples, animals, instrumentation, computing resources, or other analysis tools",
            Roles::Software => "Programming, software development; designing computer programs; implementation of the computer code and supporting algorithms; testing of existing code components.",
            Roles::Supervision => "Oversight and leadership responsibility for the research activity planning and execution, including mentorship external to the core team.",
            Roles::Validation => "Verification, whether as a part of the activity or separate, of the overall replication/reproducibility of results/experiments and other research outputs.",
            Roles::Visualization => "Preparation, creation and/or presentation of the published work, specifically visualization/data presentation.",
            Roles::WritingOriginalDraft => "Preparation, creation and/or presentation of the published work, specifically writing the initial draft (including substantive translation).",
            Roles::WritingReviewEditing => "Preparation, creation and/or presentation of the published work by those from the original research group, specifically critical review, commentary or revision – including pre- or post-publication stages.",
        };

        value.to_string()
    }
}
