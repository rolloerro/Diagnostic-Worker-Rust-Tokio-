use tokio::time::{sleep, Duration};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use log::{info, error};

#[derive(Serialize, Deserialize, Debug)]
struct MedicalRecord {
    patient_id: u32,
    risk_score: f32,
    notes: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("Diagnostic Worker started!");

    // Пример обработки 5 "тяжёлых" записей
    let records = vec![
        MedicalRecord { patient_id: 1, risk_score: 0.0, notes: "".to_string() },
        MedicalRecord { patient_id: 2, risk_score: 0.0, notes: "".to_string() },
        MedicalRecord { patient_id: 3, risk_score: 0.0, notes: "".to_string() },
        MedicalRecord { patient_id: 4, risk_score: 0.0, notes: "".to_string() },
        MedicalRecord { patient_id: 5, risk_score: 0.0, notes: "".to_string() },
    ];

    // Асинхронная обработка
    for record in records {
        tokio::spawn(async move {
            match process_record(record).await {
                Ok(_) => info!("Record processed successfully"),
                Err(e) => error!("Error processing record: {:?}", e),
            }
        });
    }

    // Чтобы воркер не завершился сразу
    sleep(Duration::from_secs(5)).await;
    Ok(())
}

async fn process_record(mut record: MedicalRecord) -> Result<()> {
    // Симуляция тяжёлой работы
    sleep(Duration::from_millis(500)).await;
    record.risk_score = rand::random::<f32>() * 100.0;
    info!("Processed record: {:?}", record);
    Ok(())
}
