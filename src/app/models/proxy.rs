#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PersonaListResponse {
    pub personas: PersonaList,
}

impl PersonaListResponse {
    pub fn new(personas: Vec<Persona>) -> Self {
        Self {
            personas: PersonaList { persona: personas },
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PersonaDetailResponse {
    pub persona: Persona,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PersonaList {
    pub persona: Vec<Persona>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Persona {
    pub anonymous_id: Option<String>,
    pub date_created: String,
    pub display_name: String,
    pub is_invisible: bool,
    pub last_authenticated: String,
    pub name: String,
    pub namespace_name: String,
    pub persona_id: String,
    pub pid_id: String,
    pub show_persona: PersonaVisibility,
    pub status: PersonaStatus,
    pub status_reason_code: String,
}

impl Persona {
    pub fn from_data(pid: impl AsRef<str>, date: impl AsRef<str>) -> Persona {
        let pid = pid.as_ref();
        Persona {
            pid_id: pid.to_owned(),
            persona_id: pid.to_owned(),
            date_created: date.as_ref().to_owned(),
            ..Default::default()
        }
    }

    pub fn with_email(mut self, has_email: bool) -> Self {
        if has_email {
            self.namespace_name = "cem_ea_id".to_owned();
        } else {
            self.namespace_name = Self::default().namespace_name;
        }

        self
    }

    pub fn with_name(mut self, user_name: Option<String>) -> Self {
        if let Some(user_name) = user_name {
            self.name = user_name.clone();
            self.display_name = user_name;
        }

        self
    }
}

impl Default for Persona {
    fn default() -> Self {
        Self {
            anonymous_id: Default::default(),
            date_created: Default::default(),
            display_name: "user".to_owned(),
            is_invisible: true,
            last_authenticated: Default::default(),
            name: "user".to_owned(),
            namespace_name: "gsp-redcrow-simpsons4".to_owned(),
            persona_id: Default::default(),
            pid_id: Default::default(),
            show_persona: Default::default(),
            status: Default::default(),
            status_reason_code: Default::default(),
        }
    }
}

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
#[serde(rename_all = "UPPERCASE")]
pub enum PersonaVisibility {
    #[default]
    Everyone,
}

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
#[serde(rename_all = "UPPERCASE")]
pub enum PersonaStatus {
    #[default]
    Active,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailLoginRequest {
    pub email: String,
    pub code_type: EmailLoginCodeType,
}
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum EmailLoginCodeType {
    Email,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoAgeRequirements {
    pub country: String,
    pub min_age_with_consent: usize,
    pub min_legal_contact_age: usize,
    pub min_legal_reg_age: usize,
}

impl Default for GeoAgeRequirements {
    fn default() -> Self {
        Self {
            country: "US".to_owned(),
            min_age_with_consent: 13,
            min_legal_contact_age: 13,
            min_legal_reg_age: 13,
        }
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkResponse {
    pub pid_game_persona_mappings: PidGamePersonaMappings,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PidGamePersonaMappings {
    pub pid_game_persona_mapping: Vec<PidGamePersonaMapping>,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PidGamePersonaMapping {
    pub new_created: bool,
    pub persona_id: String,
    pub persona_namespace: String,
    pub pid_game_persona_mapping_id: String,
    pub pid_id: String,
    pub status: PersonaStatus,
}
