// Course Articulation Matrix (CAM) service
use sea_orm::DatabaseConnection;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CamEntry {
    pub co_number: i32,
    pub co_description: String,
    pub po_mappings: Vec<i32>, // PO numbers with correlation
    pub total_marks: i32,
    pub bloom_distribution: std::collections::HashMap<String, i32>,
}

pub struct CamService {
    db: DatabaseConnection,
}

impl CamService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Generate Course Articulation Matrix for a set of questions
    pub async fn generate_cam(
        &self,
        course_id: Uuid,
        question_ids: Vec<Uuid>,
    ) -> Result<Vec<CamEntry>, String> {
        // TODO: Implement CAM generation
        // 1. Group questions by CO
        // 2. For each CO:
        //    - Get PO mappings from co_po_mapping table
        //    - Calculate total marks
        //    - Calculate bloom level distribution
        // 3. Return structured CAM data

        Ok(Vec::new())
    }

    /// Format CAM as HTML table for PDF inclusion
    pub fn format_cam_as_table(&self, cam_data: Vec<CamEntry>) -> String {
        // TODO: Generate HTML table
        // Columns: CO | Description | Questions | Bloom Levels | PO1-PO12 | Total Marks
        
        let mut html = String::from("<table border='1'>");
        html.push_str("<tr><th>CO</th><th>Description</th><th>Bloom Levels</th>");
        
        // Add PO columns
        for po in 1..=12 {
            html.push_str(&format!("<th>PO{}</th>", po));
        }
        html.push_str("<th>Total Marks</th></tr>");

        // Add rows
        for entry in cam_data {
            html.push_str(&format!("<tr><td>CO{}</td>", entry.co_number));
            html.push_str(&format!("<td>{}</td>", entry.co_description));
            
            // Bloom levels
            let bloom_str: Vec<String> = entry.bloom_distribution
                .iter()
                .map(|(level, count)| format!("{}:{}", level, count))
                .collect();
            html.push_str(&format!("<td>{}</td>", bloom_str.join(", ")));
            
            // PO mappings
            for po in 1..=12 {
                if entry.po_mappings.contains(&po) {
                    html.push_str("<td>✓</td>");
                } else {
                    html.push_str("<td></td>");
                }
            }
            
            html.push_str(&format!("<td>{}</td></tr>", entry.total_marks));
        }
        
        html.push_str("</table>");
        html
    }
}
