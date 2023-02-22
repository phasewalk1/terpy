use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// An ORM insertable cannibanoid screen
#[derive(Debug, Deserialize, Serialize, Insertable)]
#[diesel(table_name = crate::auto::cannibanoid_screen_t)]
pub struct InsertableCannibanoidScreen {
    pub grower_id: String,
    pub batch_id: String,
    pub cbc: f32,
    pub cbd: f32,
    pub cbda: f32,
    pub cbdv: f32,
    pub cbg: f32,
    pub cbga: f32,
    pub cbn: f32,
    pub d9thc: f32,
    pub d8thc: f32,
    pub thcv: f32,
    pub thca: f32,
}

impl From<prostgen::grower::NewCannibanoidScreen> for InsertableCannibanoidScreen {
    fn from(screen: prostgen::grower::NewCannibanoidScreen) -> Self {
        let grower_id: String = screen.grower_id;
        let batch_id: String = screen.batch_id;
        let cbc: f32 = screen.cbc;
        let cbd: f32 = screen.cbd;
        let cbda: f32 = screen.cbda;
        let cbdv: f32 = screen.cbdv;
        let cbg: f32 = screen.cbg;
        let cbga: f32 = screen.cbga;
        let cbn: f32 = screen.cbn;
        let d9thc: f32 = screen.d9thc;
        let d8thc: f32 = screen.d8thc;
        let thcv: f32 = screen.thcv;
        let thca: f32 = screen.thca;
        return InsertableCannibanoidScreen {
            grower_id,
            batch_id,
            cbc,
            cbd,
            cbda,
            cbdv,
            cbg,
            cbga,
            cbn,
            d9thc,
            d8thc,
            thcv,
            thca,
        };
    }
}

/// An ORM searchable cannibanoid screen
#[derive(Debug, Deserialize, Serialize, Queryable)]
#[diesel(table_name = crate::auto::cannibanoid_screen_t)]
pub struct SearchableCannibanoidScreen {
    pub id: i32,
    pub grower_id: String,
    pub batch_id: String,
    pub cbc: f32,
    pub cbd: f32,
    pub cbda: f32,
    pub cbdv: f32,
    pub cbg: f32,
    pub cbga: f32,
    pub cbn: f32,
    pub d9thc: f32,
    pub d8thc: f32,
    pub thcv: f32,
    pub thca: f32,
}

impl From<SearchableCannibanoidScreen> for prostgen::grower::CannibanoidScreen {
    fn from(screen: SearchableCannibanoidScreen) -> Self {
        let id: String = screen.id.to_string();
        let grower_id: String = screen.grower_id;
        let batch_id: String = screen.batch_id;
        let cbc: f32 = screen.cbc;
        let cbd: f32 = screen.cbd;
        let cbda: f32 = screen.cbda;
        let cbdv: f32 = screen.cbdv;
        let cbg: f32 = screen.cbg;
        let cbga: f32 = screen.cbga;
        let cbn: f32 = screen.cbn;
        let d9thc: f32 = screen.d9thc;
        let d8thc: f32 = screen.d8thc;
        let thcv: f32 = screen.thcv;
        let thca: f32 = screen.thca;
        return Self {
            id,
            grower_id,
            batch_id,
            cbc,
            cbd,
            cbda,
            cbdv,
            cbg,
            cbga,
            cbn,
            d9thc,
            d8thc,
            thcv,
            thca,
        };
    }
}

impl From<prostgen::grower::CannibanoidScreen> for SearchableCannibanoidScreen {
    fn from(screen: prostgen::grower::CannibanoidScreen) -> Self {
        let id: i32 = screen.id.parse().unwrap();
        let grower_id: String = screen.grower_id;
        let batch_id: String = screen.batch_id;
        let cbc: f32 = screen.cbc;
        let cbd: f32 = screen.cbd;
        let cbda: f32 = screen.cbda;
        let cbdv: f32 = screen.cbdv;
        let cbg: f32 = screen.cbg;
        let cbga: f32 = screen.cbga;
        let cbn: f32 = screen.cbn;
        let d9thc: f32 = screen.d9thc;
        let d8thc: f32 = screen.d8thc;
        let thcv: f32 = screen.thcv;
        let thca: f32 = screen.thca;
        return SearchableCannibanoidScreen {
            id,
            grower_id,
            batch_id,
            cbc,
            cbd,
            cbda,
            cbdv,
            cbg,
            cbga,
            cbn,
            d9thc,
            d8thc,
            thcv,
            thca,
        };
    }
}

/// An ORM insertable terpenoid screen
#[derive(Debug, Deserialize, Serialize, Insertable)]
#[diesel(table_name = crate::auto::terpenoid_screen_t)]
pub struct InsertableTerpenoidScreen {
    pub grower_id: String,
    pub batch_id: String,
    pub a_bisabolol: f32,
    pub a_humulene: f32,
    pub a_pinene: f32,
    pub a_terpinene: f32,
    pub b_caryophyllene: f32,
    pub b_myrcene: f32,
    pub b_pinene: f32,
    pub camphene: f32,
    pub caryophyllene_oxide: f32,
    pub delta_3_carene: f32,
    pub gamma_terpinene: f32,
    pub geraniol: f32,
    pub guaiol: f32,
    pub isopulegol: f32,
    pub linalool: f32,
    pub trans_nerolidol: f32,
    pub ocimene: f32,
    pub p_cymene: f32,
    pub eucalyptol: f32,
    pub terpinolene: f32,
}

impl From<prostgen::grower::NewTerpenoidScreen> for InsertableTerpenoidScreen {
    fn from(screen: prostgen::grower::NewTerpenoidScreen) -> Self {
        return Self {
            grower_id: screen.grower_id,
            batch_id: screen.batch_id,
            a_bisabolol: screen.a_bisabolol,
            a_humulene: screen.a_humulene,
            a_pinene: screen.a_pinene,
            a_terpinene: screen.a_terpinene,
            b_caryophyllene: screen.b_caryophyllene,
            b_myrcene: screen.b_myrcene,
            b_pinene: screen.b_pinene,
            camphene: screen.camphene,
            caryophyllene_oxide: screen.caryophyllene_oxide,
            delta_3_carene: screen.delta_3_carene,
            gamma_terpinene: screen.gamma_terpinene,
            geraniol: screen.geraniol,
            guaiol: screen.guaiol,
            isopulegol: screen.isopulegol,
            linalool: screen.linalool,
            trans_nerolidol: screen.trans_nerolidol,
            ocimene: screen.ocimene,
            p_cymene: screen.p_cymene,
            eucalyptol: screen.eucalyptol,
            terpinolene: screen.terpinolene,
        };
    }
}

/// An ORM searchable terpenoid screen
#[derive(Debug, Deserialize, Serialize, Queryable)]
#[diesel(table_name = crate::auto::terpenoid_screen_t)]
pub struct SearchableTerpenoidScreen {
    pub id: i32,
    pub grower_id: String,
    pub batch_id: String,
    pub a_bisabolol: f32,
    pub a_humulene: f32,
    pub a_pinene: f32,
    pub a_terpinene: f32,
    pub b_caryophyllene: f32,
    pub b_myrcene: f32,
    pub b_pinene: f32,
    pub camphene: f32,
    pub caryophyllene_oxide: f32,
    pub delta_3_carene: f32,
    pub gamma_terpinene: f32,
    pub geraniol: f32,
    pub guaiol: f32,
    pub isopulegol: f32,
    pub linalool: f32,
    pub trans_nerolidol: f32,
    pub ocimene: f32,
    pub p_cymene: f32,
    pub eucalyptol: f32,
    pub terpinolene: f32,
}

impl From<SearchableTerpenoidScreen> for prostgen::grower::TerpenoidScreen {
    fn from(screen: SearchableTerpenoidScreen) -> Self {
        return Self {
            id: screen.id.to_string(),
            grower_id: screen.grower_id,
            batch_id: screen.batch_id,
            a_bisabolol: screen.a_bisabolol,
            a_humulene: screen.a_humulene,
            a_pinene: screen.a_pinene,
            a_terpinene: screen.a_terpinene,
            b_caryophyllene: screen.b_caryophyllene,
            b_myrcene: screen.b_myrcene,
            b_pinene: screen.b_pinene,
            camphene: screen.camphene,
            caryophyllene_oxide: screen.caryophyllene_oxide,
            delta_3_carene: screen.delta_3_carene,
            gamma_terpinene: screen.gamma_terpinene,
            geraniol: screen.geraniol,
            guaiol: screen.guaiol,
            isopulegol: screen.isopulegol,
            linalool: screen.linalool,
            trans_nerolidol: screen.trans_nerolidol,
            ocimene: screen.ocimene,
            p_cymene: screen.p_cymene,
            eucalyptol: screen.eucalyptol,
            terpinolene: screen.terpinolene,
        };
    }
}

/// An ORM insertable test result
#[derive(Debug, Deserialize, Serialize, Insertable)]
#[diesel(table_name = crate::auto::test_results_t)]
pub struct InsertableTestResults {
    pub grower_id: String,
    pub batch_id: String,
    pub cann: i32,
    pub terp: i32,
}

/// An ORM searchable test result
#[derive(Debug, Deserialize, Serialize, Queryable)]
#[diesel(table_name = crate::auto::test_results_t)]
pub struct SearchableTestResults {
    pub id: i32,
    pub grower_id: String,
    pub batch_id: String,
    pub cann: i32,
    pub terp: i32,
}
