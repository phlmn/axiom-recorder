use crate::{graphical::ui_lib::*, util::error::ResN};
use glium::Surface;

/// A generic list container. If you want to draw multiple things, use this.
impl<S> Drawable<S> for Vec<&dyn Drawable<S>>
where
    S: Surface,
{
    fn draw(&self, params: &mut DrawParams<'_, S>, sp: SpatialProperties) -> ResN {
        for drawable in self {
            drawable.draw(params, sp.clone())?;
        }

        Ok(())
    }
}
