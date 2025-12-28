use anyhow::Result;
use mpris::{PlayerFinder, Player, PlaybackStatus};
use std::time::Duration;

/// Information about the currently playing track
#[derive(Debug, Clone, Default)]
pub struct TrackInfo {
    pub title: String,
    pub artist: String,
    pub position: Duration,
    pub length: Duration,
    #[allow(dead_code)]
    pub status: String,
}

impl TrackInfo {
    pub fn progress_percent(&self) -> f32 {
        if self.length.as_secs() == 0 {
            return 0.0;
        }
        (self.position.as_secs_f32() / self.length.as_secs_f32() * 100.0).min(100.0)
    }
}

pub struct MprisClient {
    player: Option<Player>,
}

impl MprisClient {
    pub fn new() -> Self {
        Self { player: None }
    }

    /// Find and connect to Spotify player
    pub fn connect(&mut self) -> Result<()> {
        let player_finder = PlayerFinder::new()?;
        
        // Try to find Spotify player
        for player in player_finder.find_all()? {
            let identity = player.identity();
            log::debug!("Found player: {}", identity);
            
            if identity.to_lowercase().contains("spotify") {
                log::info!("Connected to Spotify player");
                self.player = Some(player);
                return Ok(());
            }
        }
        
        // If no Spotify found, try to use any available player
        if let Ok(player) = player_finder.find_active() {
            log::info!("Connected to active player: {}", player.identity());
            self.player = Some(player);
            return Ok(());
        }
        
        Err(anyhow::anyhow!("No MPRIS player found"))
    }

    /// Get current track information
    pub fn get_track_info(&self) -> Result<TrackInfo> {
        let player = self.player.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not connected to player"))?;

        let metadata = player.get_metadata()?;
        let position = player.get_position().unwrap_or(Duration::ZERO);
        let status = player.get_playback_status().unwrap_or(PlaybackStatus::Stopped);

        Ok(TrackInfo {
            title: metadata.title().unwrap_or("Unknown").to_string(),
            artist: metadata.artists()
                .and_then(|a| a.first().map(|s| s.to_string()))
                .unwrap_or_else(|| "Unknown Artist".to_string()),
            position,
            length: metadata.length().unwrap_or(Duration::ZERO),
            status: format!("{:?}", status),
        })
    }

    /// Play the next track
    pub fn next(&self) -> Result<()> {
        let player = self.player.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not connected to player"))?;
        player.next()?;
        Ok(())
    }

    /// Play the previous track
    pub fn previous(&self) -> Result<()> {
        let player = self.player.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not connected to player"))?;
        player.previous()?;
        Ok(())
    }

    /// Set volume (0.0 to 1.0)
    pub fn set_volume(&self, volume: f64) -> Result<()> {
        let player = self.player.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not connected to player"))?;
        player.set_volume(volume.clamp(0.0, 1.0))?;
        Ok(())
    }

    /// Get current volume (0.0 to 1.0)
    pub fn get_volume(&self) -> Result<f64> {
        let player = self.player.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not connected to player"))?;
        Ok(player.get_volume()?)
    }

    /// Play/pause toggle
    pub fn play_pause(&self) -> Result<()> {
        let player = self.player.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not connected to player"))?;
        player.play_pause()?;
        Ok(())
    }
}
