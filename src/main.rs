use leptos::*;

#[component]
fn RatioTable() -> impl IntoView {
    let init_val = 1;
    let (value, set_value) = create_signal(init_val);

    view! {
        <div>
            <input type="range" name="hit_count" value={move||init_val} min={move||init_val} max=20 on:input=move |ev|{
                let ev = event_target_value(&ev);
                let int_val = ev.parse::<i32>().unwrap_or(1);
                set_value.update(|v| {
                    *v = int_val;
                });
            }
            />
            <label for="hit_count">hits {value}</label>
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
        let mut classes = vec!["has-text-right"];
        if percentage() > 70.0 {
            classes.push("is-success");
        }

        classes.join(" ")
    };

    view! {
        <td class="has-text-right" class={class}>{move || {
            format!("{:.2}%", percentage())
        }}</td>
    }
}

#[component]
fn TableBody(hits: ReadSignal<i32>) -> impl IntoView {
    let table_data = no_particle_table();

    view! {
        <th rowspan="6" scope="rowgroup" class="is-centered is-vcentered">精錬ランク</th>
        {table_data.into_iter().enumerate().map(
            |(i, row)| {
                view! {
                    <tr>
                        <th scope="row" class="is-info">R{i+1}</th>
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
        <table class="table is-striped is-hoverable is-bordered">
            <thead>
                <tr>
                    <th scope="colgroup" colspan="2"  />
                    <th scope="colgroup" colspan="10" class="has-text-centered">会心率</th>
                </tr>
                <tr>
                    <th />
                    <th />
                        {
                            (10..=100).step_by(10).map(|percent| {
                                view!{
                                    <th scope="col" class="is-info has-text-right">{percent}%</th>
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
