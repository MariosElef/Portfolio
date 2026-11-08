use crate::{
    models::{CaseStudyDocument, StoredBlock},
    seeds::blocks::{faq, heading, list, text},
};

/// The case study at `/projects/pygame`.
pub fn study() -> CaseStudyDocument {
    CaseStudyDocument {
        project: "pygame".into(),
        blocks: vec![
            text(
                "A side-scrolling platformer across five handcrafted levels: run, \
                 jump between moving platforms, dodge lava, collect coins and reach \
                 the exit door, while enemies **patrol until they notice the player, \
                 then give chase**. Every level was made in the game's **own level \
                 editor**.",
            ),
            heading("What I built"),
            list(&[
                "**Physics from scratch.** Gravity, jumping and collision are worked \
                 out every frame at 60 frames a second, checking the two axes \
                 separately so the player never snags on a corner.",
                "**Enemies that react.** Two kinds of enemy measure their distance to \
                 the player every frame. The green blob notices at 150 pixels and \
                 chases fast; the red chaser notices from 200 pixels but chases \
                 slower, persistent rather than overwhelming.",
                "**Moving platforms.** Platforms that slide sideways or rise and \
                 fall, carrying the player along as they move.",
                "**A level editor.** Click through the tiles of a 20 by 20 grid and \
                 save the level to disk; the game loads the levels in order.",
                "**Everything a game needs.** A main menu, pause, game over and \
                 victory screens, music and sound effects.",
            ]),
            heading("How an enemy decides"),
            StoredBlock::Code {
                lang: "python".into(),
                code: "distance_to_player = math.sqrt((self.rect.centerx - player.rect.centerx)**2 +\n                              (self.rect.centery - player.rect.centery)**2)\n\nif distance_to_player < self.chase_distance and game_over == 0:\n    if player.rect.centerx < self.rect.centerx:\n        dx = -self.chase_speed\n        self.move_direction = -1\n    elif player.rect.centerx > self.rect.centerx:\n        dx = self.chase_speed\n        self.move_direction = 1\nelse:\n    dx = self.move_direction * self.patrol_speed\n    self.move_counter += self.patrol_speed\n    if abs(self.move_counter) > self.patrol_distance:\n        self.move_direction *= -1\n        self.move_counter = 0".into(),
            },
            heading("Questions about the design"),
            faq(&[
                (
                    "Why work out the physics by hand?",
                    "Pygame draws the screen and plays sound, but leaves movement to \
                     the game. Writing gravity, jumping and collision directly keeps \
                     every rule visible and easy to tune: a jump starts at 15 pixels \
                     a frame upward, and gravity takes one away each frame.",
                ),
                (
                    "Why measure distance in a circle?",
                    "The straight-line distance, rather than only the horizontal gap, \
                     lets an enemy notice the player above or below it too, the way \
                     something actually looking would.",
                ),
                (
                    "How are levels stored?",
                    "Each level is a 20 by 20 grid of numbers, one per tile, saved \
                     with Python's pickle. The editor writes it; the game reads it \
                     and places a tile, an enemy, a platform or a coin for each \
                     number.",
                ),
            ]),
        ],
    }
}
