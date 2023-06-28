use leptos::*;
use leptos_router::*;

#[component]
pub(crate) fn TopNav(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="flex justify-center">
            <nav class="max-w-xl">
                <ul class="list-inside grid grid-cols-3">
                    <li class="p-5 flex justify-center">
                        <A class="text-gray-200" href="/">"Teratas"</A>
                        </li>
                    <li class="p-5 flex justify-center">
                        <A class="text-gray-200" href="/blogs">"Senarai Hantaran"</A>
                    </li>
                    <li class="p-5 flex justify-center">
                        <a class="text-gray-200" href="https://github.com/afiqzx/lekiu">"Hubungi Afiq"</a>
                    </li>
                </ul>
            </nav>
        </div>
    }

}

