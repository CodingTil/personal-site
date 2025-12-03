---
slug: oceancurrents
image: <img loading="lazy" src="images/projects/OceanCurrents/thumbnail.webp" alt="Examination of the North Atlantic cirulation"/>
title: Analyse von Meeresströmungen
color: bg-blue-900
tagline: Untersuchung der Nordatlantikzirkulation mit der Google Earth Engine
repository_url: https://github.com/CodingTil/ocean_currents
date_range: Oktober 2021
skills: [python, javascript, google earth engine]
filters: [python, javascript, google earth engine]
coauthors:
  - name: Leonie Gellweiler
  - name: Sven Neubauer
---
_Dieses Projekt ist ein Teil des Kurses [Global change - a data-driven approach](https://www.hc.rwth-aachen.de/cms/HC/Events/HCSS/HCSS2021/~ntcpe/GlobalChange/lidx/1/) an der [RWTH Aachen University Honors College Summer School 2021](https://www.hc.rwth-aachen.de/cms/hc/events/hcss/~ursik/hcss2021/?lidx=1)._

# Die Rolle der Nordatlantikzirkulation für das Klima in Europa
<img loading="lazy" src="images/projects/OceanCurrents/gulf_stream_visu.webp" alt="Nordatlantikzirkulation"/>

Die Nordatlantikzirkulation, insbesondere der Golfstrom, spielt eine entscheidende Rolle bei der Prägung des Klimas in West- und Mitteleuropa. Dieses System transportiert warmes Wasser vom Golf von Mexiko in den Nordatlantik, beeinflusst die Lufttemperatur darüber und wirkt sich dadurch auf das Wetter Europas durch Westwinde aus. Wenn dieses Wasser in der Nähe Islands ankommt, kühlt es ab, sinkt und fließt zurück in Richtung Golf von Mexiko.

Mit dem fortschreitenden Klimawandel besteht die Befürchtung, dass diese Zirkulation schwächer wird, was paradoxerweise zu einer Abkühlung Europas inmitten der globalen Erwärmung führen könnte. Dies könnte durch Faktoren wie das Schmelzen des grönländischen Eisschilds beeinflusst werden, welches Süßwasser in den Nordatlantik einbringt und die Dichte und Stärke der Zirkulation stört.

# Projektziel
Unser Projekt zielt darauf ab, den aktuellen Zustand der Nordatlantikzirkulation und ihren Einfluss auf das Klima Europas zu untersuchen. Mit Datensätzen aus dem [Google Earth Engine](https://earthengine.google.com/), untersuchen wir, ob bereits heute Anzeichen für eine Schwächung der Nordatlantikzirkulation vorliegen. Unsere Schwerpunktgebiete sind der sich erwärmende Golf von Mexiko und der sich abkühlende Nordatlantik, gekennzeichnet als die rote (warme) und blaue (kalte) Region in unserer Studie.

<img loading="lazy" src="images/projects/OceanCurrents/regions_of_interest.webp" alt="Interessensregionen"/>

# Verwendete Daten
<img loading="lazy" src="images/projects/OceanCurrents/datasets.webp" alt="HYCOM"/>

Für unsere Analyse verwenden wir folgende Datensätze:
 - HYCOM: Hybrid Coordinate Ocean Model, umfasst Wassertemperatur und -salzgehalt
 - HYCOM: Hybrid Coordinate Ocean Model, Wassergeschwindigkeit

## Details zu den Datensätzen
| Eigenschaft | Beschreibung |
| --- | --- |
| Räumliche Auflösung | 1/12° |
| Zeitliche Auflösung | 02.10.1992 bis heute<br>Alle 3 Stunden |
| Kategorien | Temperatur (C)<br>Salzgehalt (psu)<br>Geschwindigkeit_x (m/s)<br>Geschwindigkeit_y (m/s) |
| Erfasste Tiefen | 0m - 500m über 40 Ebenen |

## Datenverarbeitung
Unser Datensatz umfasste ursprünglich über 10600 Tage an Bildern, 8 Bilder pro Tag und 4*40 Bänder pro Bild. Um diese große Datenmenge zu bewältigen, haben wir:
 - Die zeitliche Auflösung auf wöchentliche Durchschnitte reduziert, was 1500 Bilder ergab
 - Techniken zur Reduzierung der räumlichen Auflösung angewendet und statistische Maße wie Minimum, Maximum, Mittelwert, Median, Varianz und Schiefe für jede Region von Interesse berechnet

Dieser Ansatz komprimierte die Daten erheblich, sodass wir pro Region etwa 1000 wöchentliche Werte erhielten, insgesamt etwa 23MB Daten pro Region.

# Analyse
## Temperatur
<img loading="lazy" src="images/projects/OceanCurrents/temperature_warm.webp" alt="Temperatur Warm"/>
<img loading="lazy" src="images/projects/OceanCurrents/temperature_cold.webp" alt="Temperatur Cold"/>

Oberflächentemperaturen spiegeln saisonale Veränderungen deutlicher wider, während tiefere Ebenen stabilere Temperaturen aufweisen. Auffällig ist, dass der Golfstrom dafür sorgt, dass die warme Region stets wärmer als die kalte Region bleibt. Im Laufe der Jahrzehnte sind die Temperaturen in der warmen Region gestiegen, während die kalte Region eine leichte Abkühlung zeigt.

## Salzgehalt
<img loading="lazy" src="images/projects/OceanCurrents/salinity_warm.webp" alt="Salzgehalt Warm"/>
<img loading="lazy" src="images/projects/OceanCurrents/salinity_cold.webp" alt="Salzgehalt Cold"/>

Die Salzgehalttrends sind ausgeprägter, wobei die warme Region sowohl in flachen als auch in tiefen Gewässern einen Anstieg des Salzgehalts erlebt. Im Gegensatz dazu nimmt der Salzgehalt in der kalten Region ab, möglicherweise aufgrund des Süßwassereintrags durch schmelzendes Eis. Diese Trends deuten auf eine Schwächung der Nordatlantikzirkulation hin.

## Geschwindigkeit
Unsere Analyse der Geschwindigkeitsdaten zeigte eine erhebliche Instabilität, insbesondere in flachen Gewässern. Daher konzentrierten wir uns auf die Geschwindigkeiten in tieferen Gewässern, gemessen als euklidische Norm der Geschwindigkeiten in X- und Y-Richtung.

<img loading="lazy" src="images/projects/OceanCurrents/velocity_warm.webp" alt="Geschwindigkeit Warm"/>
<img loading="lazy" src="images/projects/OceanCurrents/velocity_cold.webp" alt="Geschwindigkeit Cold"/>

Die Geschwindigkeitsdaten zeigten eine leichte Verringerung der Wasserbewegung in der warmen Region und eine geringfügige Zunahme in der kalten Region. Jedoch macht die minimale Skala dieser Veränderungen (in m/s) diese Daten weniger aussagekräftig als die Temperatur- und Salzgehaltbefunde.

## Korrelation von Temperatur und Salzgehalt über verschiedene Tiefen
<img loading="lazy" src="images/projects/OceanCurrents/correlation_warm.webp" alt="Korrelation Warm"/>
<img loading="lazy" src="images/projects/OceanCurrents/correlation_cold.webp" alt="Korrelation Cold"/>

Bemerkenswert ist die unterschiedliche Korrelation zwischen Temperatur und Salzgehalt in verschiedenen Tiefen. In beiden Regionen zeigen diese Faktoren in flacheren Gewässern eine inverse Korrelation, eine direkte Korrelation in etwa 600m Tiefe und keine signifikante Korrelation mehr jenseits von 1200m Tiefe. Die Gründe für diese Muster bleiben weiteren Untersuchungen vorbehalten.

# Schlussfolgerung
Unsere Ergebnisse deuten darauf hin, dass in der kälteren Region des Nordatlantiks die Temperatur und der Salzgehalt sinken, während in der wärmeren Region des Golf von Mexiko die Temperatur und der Salzgehalt zunehmen. Diese Trends stimmen mit der Hypothese einer Abschwächung der Nordatlantikzirkulation überein.

# Weiterführende Literatur und Referenzen
 - [Caesar, Levke, Stefan Rahmstorf, Alexander Robinson, Georg Feulner, and V. Saba. "Observed fingerprint of a weakening Atlantic Ocean overturning circulation." Nature 556, no. 7700 (2018): 191-196](https://www.nature.com/articles/s41586-018-0006-5)
 - [Chen, Xianyao, and Ka-Kit Tung. "Global surface warming enhanced by weak Atlantic overturning circulation." Nature 559, no. 7714 (2018): 387-391.](https://www.nature.com/articles/s41586-018-0320-y)
