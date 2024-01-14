use leptos::*;

#[component]
fn RatioTable() -> impl IntoView {
    let init_val = 1;
    let (value, set_value) = create_signal(init_val);
    let on_slider_change = move |ev| {
        let ev = event_target_value(&ev);
        let int_val = ev.parse::<i32>().unwrap_or(1);
        set_value.update(|v| {
            *v = int_val;
        });
    };

    let on_input_change = move |ev| {
        let value = event_target_value(&ev);
        if value.is_empty() {
            return;
        }
        let value = value.parse::<i32>().unwrap_or(1);
        set_value.set(value);
    };

    view! {
        <div>
            # of hits
        </div>
        <div>
            <input
                type="number"
                prop:value=value
                on:input=on_input_change
            />
        </div>
        <div>
            <input
                type="range"
                name="hit_count"
                prop:value={value}
                min={init_val}
                max=20
                on:input=on_slider_change
            />
            //<label for="hit_count">hits {value}</label>
        </div>
        <Table hits={value}/>
    }
}

fn no_particle_table() -> [[f64; 10]; 5] {
    // Crit rates for 10%, 20%, ..., 100%.
    let crit_rates: [f64; 10] = std::array::from_fn(|i| (10 * (i + 1)) as f64);
    // Particle emit rates for favonius weapons, per crit. From R1 (60%) to R5 (100%).
    let partical_emit_rates: [f64; 5] = std::array::from_fn(|i| 0.6 + 0.1 * i as f64);
    partical_emit_rates.map(|er| crit_rates.map(|cr| 1.0 - cr / 100.0 * er))
}

#[component]
fn PercentTableData(hits: ReadSignal<i32>, non_emit_prob: f64) -> impl IntoView {
    let percentage = move || 100.0 * (1.0 - non_emit_prob.powi(hits.get()));

    let class = move || {
        let mut classes = vec!["text-right"];
        if percentage() > 70.0 {
            classes.push("text-bg-success");
        }

        classes.join(" ")
    };

    view! {
        <td class={class}>{move || {

            let mut percentage = format!("{:.1}", percentage());
            if percentage == "100.0" {
                percentage = "100".to_string();
            }
            format!("{percentage}%")
        }}</td>
    }
}

#[component]
fn TableBody(hits: ReadSignal<i32>) -> impl IntoView {
    let another_table = no_particle_table();

    view! {
        {another_table.into_iter().enumerate().map(
            |(i, row)| {
                view! {
                    <tr>
                        <th scope="row" class="text-bg-info text-right">R{i+1}</th>
                        {row.into_iter().map(
                            |item| {
                                view! {
                                    <PercentTableData hits=hits non_emit_prob=item />
                                }
                            }

                        ).collect_view()}
                    </tr>
                }
            }
        ).collect_view()}
    }
}

#[component]
fn Table(hits: ReadSignal<i32>) -> impl IntoView {
    view! {
        <table class="table table-striped table-bordered">
            <thead>
                <tr>
                    <th scope="colgroup" colspan="2"  />
                    <th scope="colgroup" colspan="10" class="text-center">会心率</th>
                </tr>
                <tr>
                    <th />
                        {
                            (10..=100).step_by(10).map(|percent| {
                                view!{
                                    <th scope="col" class="text-bg-info text-right">{percent}%</th>
                                }
                            }
                            ).collect_view()
                        }
                </tr>
            </thead>
            <tbody>
                <TableBody hits=hits />
            </tbody>
        </table>
    }
}

fn main() {
    mount_to_body(|| {
        view! {
            <RatioTable />
        }
    })
}
