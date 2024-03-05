wit_bindgen::generate!({
    path: "wit",
    world: "proxy",
    ownership: Borrowing { duplicate_if_necessary: true },
});
