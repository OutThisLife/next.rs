// struct Component {
//   element: String,
//   did_mount: Option<fn(&Component)>,
//   will_unmount: Option<fn(&Component)>,
//   did_update: Option<fn(&Component)>,
//   only_update_for_keys: Vec<String>,
// }

// impl Component {
//   fn connected_callback(&self) {
//     if let Some(did_mount) = self.did_mount {
//       did_mount(self)
//     }
//   }
// }

pub fn render(idek: &str) -> String {
  format!("<h1>Isomdorphic-ish? {}</h1>", idek)
  // js! {
  //   window.customElements.define("my-app", class extends HTMLElement {
  //     constructor() {
  //       super()
  //     }

  //     static get observedAttributes() {
  //       return []
  //     }

  //     connectedCallback() {
  //       const shadow = this.attachShadow({ mode: "open" });
  //       shadow.innerHTML = @{"
  //       <style>
  //       h1 {
  //         color: red;
  //       }
  //       </style>

  //       <h1>Title</h1>
  //       "};

  //       console.log("dx")
  //     }

  //     disconnectedCallback() {
  //       console.log("bye")
  //     }

  //     attributeChangeCallback(attr, oldVal, newVal) {
  //       console.log(attr, oldVal, newVal)
  //     }
  //   })
  // }
}