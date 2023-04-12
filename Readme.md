<h1>How scores are calculated</h1>

<h2>Factors considered in the NZPRS formula</h2>
    <ol>
        <li>Position ranking <strong>(P<sub>p</sub>)</strong>:<br>
            The value of a participant's effort in a competition relative to the other participants in the same competition.
            This is calculated from the actual total scores from the competition (Gap or other scoring formula).</li>
        <li>Competition ranking <strong>(P<sub>q</sub>, P<sub>n</sub>, T<sub>a</sub>)</strong>:<br>
            The value of the competition relative to other competitions in the same ranking (using the competitions in
            the last ranking prior to the competition as benchmark).
        <li>Time devaluation <strong>(T<sub>d</sub>)</strong>:<br>
            The value of the competition should decrease over time, otherwise we would have a "all time best" ranking
            instead of a current ranking.
        <li>The number of results that should count for a participant in the ranking. It is sum of the points of 4 best
            competitions in the last 3 years.
    </ol>
    <h2>The actual NZPRS formula:</h2>
    <math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
    <semantics>
    <mrow>
    <mi mathvariant="italic">NZPR</mi>
    <mo stretchy="false">=</mo>
    <mrow>
    <msub>
    <mi>P</mi>
    <mi>p</mi>
    </msub>
    <mo stretchy="false">∗</mo>
    <msub>
    <mi>P</mi>
    <mi>q</mi>
    </msub>
    <mo stretchy="false">∗</mo>
    <msub>
    <mi>P</mi>
    <mi>n</mi>
    </msub>
    <mo stretchy="false">∗</mo>
    <msub>
    <mi>T</mi>
    <mi>a</mi>
    </msub>
    <mo stretchy="false">∗</mo>
    <msub>
    <mi>T</mi>
    <mi>d</mi>
    </msub>
    </mrow>
    </mrow>
    <annotation encoding="StarMath 5.0">NZPR = P_p*P_q*P_n*T_a*T_d</annotation>
    </semantics>
    </math>
    <p>To make the points more readable it is multiplied by 100 and rounded to 1 decimal.</p>
    <math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
    <semantics>
    <mrow>
    <mrow>
    <mi mathvariant="italic">NZPR</mi>
    <mo stretchy="false">=</mo>
    <mi mathvariant="italic">round</mi>
    </mrow>
    <mrow>
    <mo fence="true" stretchy="false">(</mo>
    <mrow>
    <mrow>
    <mrow>
    <msub>
    <mi>P</mi>
    <mi>p</mi>
    </msub>
    <mo stretchy="false">∗</mo>
    <msub>
    <mi>P</mi>
    <mi>q</mi>
    </msub>
    <mo stretchy="false">∗</mo>
    <msub>
    <mi>P</mi>
    <mi>n</mi>
    </msub>
    <mo stretchy="false">∗</mo>
    <msub>
    <mi>T</mi>
    <mi>a</mi>
    </msub>
    <mo stretchy="false">∗</mo>
    <msub>
    <mi>T</mi>
    <mi>d</mi>
    </msub>
    <mo stretchy="false">∗</mo>
    <mn>100,</mn>
    </mrow>
    <mn>1</mn>
    </mrow>
    </mrow>
    <mo fence="true" stretchy="false">)</mo>
    </mrow>
    </mrow>
    <annotation encoding="StarMath 5.0">NZPR = round(P_p*P_q*P_n*T_a*T_d*100, 1)</annotation>
    </semantics>
    </math>
    <p>The participant's place in a given ranking (at a ranking date) is decided by the sum of the top 4 results in the last 3
        years.</p>
    <p>The <strong>competition ranking factor</strong> is based on <strong>real differences</strong> in the number of top-ranked
        pilots participating and the number of pilots participating in the competition, relative to the number of pilots in the
        ranking and in the average competition for the given ranking.</p>
    <h2>Pilot Points (P<sub>p</sub>)</h2>
    <p>The value of a person's effort in a competition relative to the other participants is calculated as a curve.
        The curve is using the pilot quality (P<sub>q</sub>) so in a competition with high ranked pilots the curve is fairly steep,
        but in competitions with lower ranked pilots it gets close to a straight line.</p>
    <p>P<sub>q</sub> has the value of 0.2 to 1.0 based on the rankings of the pilots in the competition.
        As the formula uses P<sub>q</sub> as power creating a curve and P<sub>q</sub> varies, the curve varies.</p>
    <p>So the formula uses the maximum value comparing the value based on the actual P<sub>q</sub> and if this was the highest
        valued competition with P<sub>q</sub> = 1.0.</p>
    <math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
    <semantics>
    <mrow>
    <mrow>
    <msub>
    <mi>P</mi>
    <mi>p</mi>
    </msub>
    <mo stretchy="false">=</mo>
    <mi mathvariant="italic">max</mi>
    </mrow>
    <mrow>
    <mo fence="true" stretchy="false">(</mo>
    <mrow>
    <mrow>
    <msup>
    <msub>
    <mi>P</mi>
    <mi mathvariant="italic">placing</mi>
    </msub>
    <mrow>
    <mo fence="true" stretchy="false">(</mo>
    <mrow>
    <mrow>
    <mn>1</mn>
    <mo stretchy="false">+</mo>
    <msub>
    <mi>P</mi>
    <mi>q</mi>
    </msub>
    </mrow>
    </mrow>
    <mo fence="true" stretchy="false">)</mo>
    </mrow>
    </msup>
    <mi>,</mi>
    <msup>
    <msub>
    <mi>P</mi>
    <mi mathvariant="italic">placing</mi>
    </msub>
    <mn>2</mn>
    </msup>
    </mrow>
    </mrow>
    <mo fence="true" stretchy="false">)</mo>
    </mrow>
    </mrow>
    <annotation encoding="StarMath 5.0">P_p=max( {P_placing}^(1+P_q), {P_placing}^2)</annotation>
    </semantics>
    </math>
    <p>Where:<br> <strong>P<sub>placing</sub></strong> is (last place - pilot place+1)/ last place
    </p>
    <h2>Competition ranking (P<sub>q</sub>, P<sub>n</sub>, T<sub>a</sub>)</h2>
    <p>In a perfect competition with all the top pilots participating competition ranking should be 1.0. So,
        what to do with all those other competitions? Winning a competition with only beginner pilots or a
        competition with only one participant should give a competition ranking close to 0.0.</p>
    <p>We use three factors to measure the value of a competition:</p>
    <ol>
        <li>The quality of the participants (<strong>P<sub>q</sub></strong>).</li>
        <li>The number of participants compared to other competitions in same ranking (<strong>P<sub>n</sub></strong>).</li>
        <li>The success of the competition (<strong>T<sub>a</sub></strong>).</li>
    </ol>
    <math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
    <semantics>
    <mrow>
    <mi mathvariant="italic">Competition</mi>
    <mrow>
    <mi mathvariant="italic">Ranking</mi>
    <mo stretchy="false">=</mo>
    <mrow>
    <msub>
    <mi>P</mi>
    <mi>q</mi>
    </msub>
    <mo stretchy="false">∗</mo>
    <msub>
    <mi>P</mi>
    <mi>n</mi>
    </msub>
    <mo stretchy="false">∗</mo>
    <msub>
    <mi>T</mi>
    <mi>a</mi>
    </msub>
    </mrow>
    </mrow>
    </mrow>
    <annotation encoding="StarMath 5.0">Competition Ranking = P_q*P_n*T_a</annotation>
    </semantics>
    </math>
    <h3>Participant quality (P<sub>q</sub>)</h3>
    <p>Presumption: A competition with maximum quality of participants would be a competition where all the top
        ranked pilots participated.</p>
    <p>To find <strong>P<sub>q</sub></strong> we use the last ranking prior to the competition and find the sum of ranking points for the top 1/2
        ranked pilots that are entered in the competition. Then we find the sum of ranking points as if those pilots
        would have been the top ranked pilots of the world. This gives us 1.0 if the top ranked pilots had actually
        entered and 0.0 if no ranked pilots are entered.</p>
    <p>To avoid <strong>P<sub>q</sub></strong> = 0 for comps with no ranked pilots set a lower limit of 0.2.</p>
    <math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
    <semantics>
    <mrow>
    <msub>
    <mi>P</mi>
    <mi>q</mi>
    </msub>
    <mo stretchy="false">=</mo>
    <mrow>
    <mrow>
    <mfrac>
    <msub>
    <mi>P</mi>
    <msub>
    <mi>q</mi>
    <mi mathvariant="italic">srp</mi>
    </msub>
    </msub>
    <msub>
    <mi>P</mi>
    <msub>
    <mi>q</mi>
    <mi mathvariant="italic">srtp</mi>
    </msub>
    </msub>
    </mfrac>
    <mo stretchy="false">∗</mo>
    <mrow>
    <mo fence="true" stretchy="false">(</mo>
    <mrow>
    <mrow>
    <mn>1</mn>
    <mo stretchy="false">−</mo>
    <msub>
    <mi>P</mi>
    <msub>
    <mi>q</mi>
    <mi mathvariant="italic">min</mi>
    </msub>
    </msub>
    </mrow>
    </mrow>
    <mo fence="true" stretchy="false">)</mo>
    </mrow>
    </mrow>
    <mo stretchy="false">+</mo>
    <msub>
    <mi>P</mi>
    <msub>
    <mi>q</mi>
    <mi mathvariant="italic">min</mi>
    </msub>
    </msub>
    </mrow>
    </mrow>
<annotation encoding="StarMath 5.0">P_q = P_{q_srp} over P_{q_srtp}* (1 - P_{q_min}) + P_{q_min}</annotation>
    </semantics>
    </math>
    <p>Where:<br>
        <strong>P<sub>q<sub>srp</sub></sub></strong> = "sum ranking points of the top 1/2 ranked participants"<br>
        <strong>P<sub>q<sub>srtp</sub></sub></strong> = "sum ranking points if they had been the top ranked pilots of the world"</br>
        <strong>P<sub>q<sub>min</sub></sub></strong> = "minimum P<sub>q</sub>"</p>
    <p>Virtually no competition will get P<sub>q</sub> = 1.0. Top competitions may get between 0.7 and 0.8 and there will be a
        difference between these.</p>
    <h3>Number of participants (P<sub>n</sub>)</h3>
    <math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
    <semantics>
    <mtable>
    <mtr>
    <mtd>
    <mrow>
    <msub>
    <mi>P</mi>
    <mi>n</mi>
    </msub>
    <mo stretchy="false">=</mo>
    <msqrt>
    <mfrac>
    <msub>
    <mi>P</mi>
    <mi mathvariant="italic">num</mi>
    </msub>
    <msub>
    <mi>P</mi>
    <mrow>
    <mi mathvariant="italic">ave</mi>
    <mn>12</mn>
    <mi mathvariant="italic">months</mi>
    </mrow>
    </msub>
    </mfrac>
    </msqrt>
    </mrow>
    </mtd>
    </mtr>
    <mtr>
    <mtd>
    <mrow>
    <mi mathvariant="italic">if</mi>
    <mrow>
    <mo fence="true" stretchy="false">(</mo>
    <mrow>
    <mrow>
    <msub>
    <mi>P</mi>
    <mi>n</mi>
    </msub>
    <mo stretchy="false">&gt;</mo>
    <msub>
    <mi>P</mi>
    <msub>
    <mi>n</mi>
    <mi mathvariant="italic">max</mi>
    </msub>
    </msub>
    </mrow>
    </mrow>
    <mo fence="true" stretchy="false">)</mo>
    </mrow>
    <mrow>
    <msub>
    <mi>P</mi>
    <mi>n</mi>
    </msub>
    <mo stretchy="false">=</mo>
    <msub>
    <mi>P</mi>
    <msub>
    <mi>n</mi>
    <mi mathvariant="italic">max</mi>
    </msub>
    </msub>
    </mrow>
    </mrow>
    </mtd>
    </mtr>
    </mtable>
    <annotation encoding="StarMath 5.0">P_n = sqrt{ P_{num} over P_{ave 12 months}} newline newline
    if (P_n &gt; P_{n_max}) P_n = P_{n_max}</annotation>
    </semantics>
    </math>
    <p>Where:<br>
        <strong>P<sub>num</sub></strong> = number of participants<br>
        <strong>P<sub>ave 12 months</sub></strong> = avgerage number of participants in competitions in the last 12 months<br>
        <strong>P<sub>n<sub>max</sub></sub></strong> = 1.2, saying that a competition with slightly more than average number of participants is a good
        benchmark.</p>
    <p>Looking at New Zealand paragliding competition data on 01/10/2017 the average number of pilots in PG XC competitions is 37.
        However since the system does not hold historic data (the initial seeding is taken from the ladder rather than competition
        data) the first few competitions scored will benefit from an elevated <strong>P<sub>n</sub></strong>. This minor aberration will
        quickly be eliminated as more competitions are scored. The <strong>P<sub>n</sub></strong> will then track the changes in competition
        popularity over time.</p>
    <h3>Success (T<sub>a</sub>)</h3>
    <p>One last thing one may consider is the success of the competition (<strong>T<sub>d</sub></strong>), ie was it a fair competition. There
        are many ways to measure this, none is very objective or accurate. As competitions in paragliding mostly involve a number
        of tasks we tend to use this as a measure of success.</p>
    <p>Td values for Paragliding XC:<br>
    <ul>
        <li>1 task: 0.4</li>
        <li>2 tasks: 0.6</li>
        <li>3 tasks: 0.8</li>
        <li>4 tasks: 0.9</li>
        <li>>4 tasks: 1.0</li>
    </ul>
    <p>This really means that a Paragliding competition has full value if there are 5 or more valid tasks.</p>
    <h2>Time devaluation (T<sub>d</sub>)</h2>
    <math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
    <semantics>
    <mrow>
    <mi mathvariant="italic">Td</mi>
    <mo stretchy="false">=</mo>
    <mfrac>
    <mn>1</mn>
    <mrow>
    <mn>1</mn>
    <mo stretchy="false">+</mo>
    <msubsup>
    <mi mathvariant="italic">Td</mi>
    <mi>a</mi>
    <mrow>
    <mo fence="true" stretchy="true">(</mo>
    <mrow>
    <mrow>
    <mrow>
    <mfrac>
    <mi mathvariant="italic">DaysSinceEndOfComp</mi>
    <mn>1096</mn>
    </mfrac>
    <mo stretchy="false">∗</mo>
    <msub>
    <mi mathvariant="italic">Td</mi>
    <mi>b</mi>
    </msub>
    </mrow>
    <mo stretchy="false">−</mo>
    <mfrac>
    <msub>
    <mi mathvariant="italic">Td</mi>
    <mi>b</mi>
    </msub>
    <mn>2</mn>
    </mfrac>
    </mrow>
    </mrow>
    <mo fence="true" stretchy="true">)</mo>
    </mrow>
    </msubsup>
    </mrow>
    </mfrac>
    </mrow>
    <annotation encoding="StarMath 5.0">Td = 1 over {1+ Td_a ^(DaysSinceEndOfComp over 1096* Td_b - Td_b over 2)}</annotation>
    </semantics>
    </math>
    <p>This gives an s-curve with x in the range 0 to 1096 (days or 3 years) and y going from 1.0 to 0.0.<br>
        <strong>T<sub>d<sub>a</sub></sub></strong> = 2, <strong>T<sub>d<sub>b</sub></sub></strong> = 20 (changing these will change shape of the s-curve).</p>
    </p>
    <h2>Overseas Competitions</h2>
    <h3>Leagues</h3>
    <p>Any overseas competition in which 6 or more New Zealand pilots compete can be declared an overseas league. In this case the
        competition is scored as if the was a local competition. The New Zealand pilots are extracted from the overall competition
        results and entered into the NZPRS in the order of their placing in the competition.</p>
    <h3>FAI Competitions</h3>
    <p>Overseas competitions with fewer than 6 New Zealand pilots competing provide a sample size which is too small to provide a
        fair ranking of pilot ability. However, if it is a FAI Cat1 or Cat2 competition the results can be included in the NZPRS through
        a points exchange rate mechanism.</p>
    <p>The WPRS points scored by the pilot in the FAI competition will be multiplied by the exchange rate to find the corresponding
        NZPRS points and included directly in the NZPRS table.</p>
    <p>The exchange rate is calculated from the average NZPRS score for New Zealand competitions in the last two years
        divided by the average WPRS score for those same competitions.</p>
    <math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
    <semantics>
    <mrow>
    <mrow>
    <mi mathvariant="italic">NZPRS</mi>
    <mo stretchy="false">=</mo>
    <mfrac>
    <msub>
    <mi mathvariant="italic">NZPRS</mi>
    <mi mathvariant="italic">ave</mi>
    </msub>
    <msub>
    <mi mathvariant="italic">WPRS</mi>
    <mrow>
    <mi mathvariant="italic">NZ</mi>
    <mi mathvariant="italic">comp</mi>
    </mrow>
    </msub>
    </mfrac>
    </mrow>
    <msub>
    <mi mathvariant="italic">WPRS</mi>
    <mi mathvariant="italic">pp</mi>
    </msub>
    </mrow>
    <annotation encoding="StarMath 5.0">NZPRS = NZPRS_ave over WPRS_{ NZ comp } WPRS_pp</annotation>
    </semantics>
    </math>
    <p>Where:<br>
        <strong>NZPRS<sub>ave</sub></strong> = average score on NZPRS for competitions in the last 2 years that are also scored on WPRS<br>
        <strong>WPRS<sub>NZ comp</sub></strong> = avgerage WPRS score for the same competitions<br>
        <strong>WPRS<sub>pp</sub></strong> = WPRS points of the pilot in the overseas competition</p>
