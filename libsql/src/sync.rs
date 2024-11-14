const DEFAULT_MAX_RETRIES: usize = 5;

use crate::Result;

pub struct SyncContext {
    pub sync_url: String,
    pub auth_token: Option<String>,
    pub max_retries: usize,
    pub durable_frame_num: u32,
    max_frame_no: u32,
    db_path: String,
}

impl SyncContext {
    pub fn new(sync_url: String, auth_token: Option<String>, db_path: impl Into<String>) -> Self {
        let mut ctx = Self {
            sync_url,
            auth_token,
            durable_frame_num: 0,
            max_retries: DEFAULT_MAX_RETRIES,
            max_frame_no: 0,
            db_path: db_path.into(),
        };

        ctx.read_and_update_metadata().unwrap();

        ctx
    }

    pub(crate) fn max_frame_no(&self) -> u32 {
        self.max_frame_no
    }

    pub(crate) fn set_max_frame_no(&mut self, max_frame_no: u32) -> Result<()> {
        // TODO: check if max_frame_no is larger than current known max_frame_no
        self.max_frame_no = max_frame_no;

        self.update_metadata()?;

        Ok(())
    }

    fn update_metadata(&self) -> Result<()> {
        let path = format!("{}-info", self.db_path);

        let contents = serde_json::to_vec(&MetadataJson {
            max_frame_no: self.max_frame_no,
        })
        .unwrap();

        std::fs::write(path, contents).unwrap();

        Ok(())
    }

    fn read_and_update_metadata(&mut self) -> Result<()> {
        let path = format!("{}-info", self.db_path);

        let contents = std::fs::read(&path).unwrap();

        let metadata = serde_json::from_slice::<MetadataJson>(&contents[..]).unwrap();

        self.max_frame_no = metadata.max_frame_no;

        Ok(())
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct MetadataJson {
    max_frame_no: u32,
}
