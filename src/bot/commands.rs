//! # Commands
//!
//! Big luca bot commands

use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone, Debug)]
#[command(
    rename = "kebab-case",
    description = "Questi comandi sono disponibili:"
)]
pub enum Command {
    #[command(description = "ottieni un'immagine del buongiorno")]
    Buongiornissimo,
    #[command(description = "ottieni un'immagine del buon pomeriggio")]
    Buonpomeriggio,
    #[command(description = "ottieni un'immagine della buona notte")]
    Buonanotte,
    #[command(description = "ottieni un'immagine del buon natale")]
    BuonNatale,
    #[command(description = "ottieni un'immagine di buona pasqua")]
    BuonaPasqua,
    #[command(description = "iscriviti ai messaggi automatici")]
    Caffeee,
    #[command(description = "disinscriviti dai messaggi automatici")]
    PuliziaKontatti,
    #[command(description = "ottieni la release attuale")]
    Release,
    #[command(description = "visualizza l'aiuto")]
    Help,
    #[command(description = "inizializza bot")]
    Start,
}
