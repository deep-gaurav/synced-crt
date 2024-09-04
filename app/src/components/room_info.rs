use leptos::*;

use crate::components::portal::Portal;
use crate::networking::room_manager::RoomManager;
use crate::MountPoints;

#[component]
pub fn RoomInfo() -> impl IntoView {
    let room_manager = expect_context::<RoomManager>();
    let room_info = room_manager.get_room_info();
    view! {
        {move || {
            let mount_points = expect_context::<MountPoints>();
            let el = mount_points.handle_point.get();
            if let Some(el) = el {
                let element: &web_sys::Element = el.as_ref();
                let element = element.clone();
                view! {
                    <Portal
                        mount=element
                        class="h-full w-full bg-black text-white flex flex-col justify-stretch items-center"
                    >
                        <div class="text-xs font-thin8 text-center">"Room"</div>
                        <div>
                            {move || match room_info
                                .with(|r| r.as_ref().map(|r| r.id.to_uppercase()))
                            {
                                Some(id) => id,
                                None => "Disconnected".to_string(),
                            }}
                        </div>
                        <hr class="border-white border-t w-full" />
                        <For
                            each=move || {
                                room_info
                                    .with(|r| r.as_ref().map(|r| r.users.clone()))
                                    .unwrap_or_default()
                            }
                            key=|user| user.id
                            children=|user| {
                                view! {
                                    <div class="text-left w-full mt-2">
                                        "> " {user.name}
                                        {match user.state {
                                            common::UserState::VideoNotSelected => "⌛",
                                            common::UserState::VideoSelected(_) => "✔️",
                                        }}
                                    </div>
                                }
                            }
                        />
                    </Portal>
                }
                    .into_view()
            } else {
                view! {}.into_view()
            }
        }}
    }
}
