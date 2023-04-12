use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    let raw = r#"
    <section class="section">
    <h1 class="title">About the New Zealand Pilot Ranking System</h1>
    <p>The New Zealand Pilot Ranking System (NZPRS) is the ranking system for New Zealand Cross Country Competition Paragliding Pilots.</p>
    <p>The NZPRS formula is based on the World Pilot Ranking System (WPRS) used by CIVL.</p>
    <p>The main aim of the NZPRS is to rank New Zealand pilots in a fair manner, so the rankings will show the
        strength of each individual pilot, based on the competitions in which they have participated.</p>
    <p>This ranking formula takes effect from Sept 1st 2017. The new ranking has been seeded with data from the 
        old ladder that was in effect prior to that date. All rankings after 1st Sept 
        will use the new formula as documented below.</p>
    <p>The most significant change from the old system is that NZPRS is based on <em>competition results</em> rather than <em>individual 
            tasks</em>. Points are awarded on the ranking table according to each pilot's placing in the competition. This encourages 
        pilots to value competition results over specific task results and means that tactics may play a more important role.</p>
    <p>NZPRS imposes no requirements on the scoring system used in each competition, since it only considers the placing of 
        pilots in the final results. The onus is on the competition director to use whatever scoring parameters are appropriate 
        (within the paragliding competition rules) to achieve a fair ranking of pilots according to their ability.</p>
    <p>NZPRS awards pilot (participant) points based on the sum of 4 best competitions in the last 3 years with time devaluation
        (T<sub>d</sub>) to reduce the value of results as they age. Time devaluation is important in the formula because the value of the 
        competition should decrease over time, otherwise we would have an "all time best in last 3 years" ranking instead of a 
        current ranking.</p>
    <p>The formula used to calculate the NZPRS ranking is documented in the Official Rules for NZ Paragliding Competitions on the 
        <a href="http://www.nzhgpa.org.nz/competitions/pg-competitions/downloads">NZHGPA website</a>.</p>
</section>
"#;
    html! { <>{ Html::from_html_unchecked(AttrValue::from(raw))}</> }
}
