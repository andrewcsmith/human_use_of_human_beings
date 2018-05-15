# removing noise; leaving information

This project takes Norbert Wiener's *Human Use of Human Beings* as a source
text and inspiration, transforming a section by removing information in each
successive paragraph. The Extermination of the American Bison is used as a
statistical model of the English language, building a transition probability
table (using a radix trie) for each n-gram. Then, a window is moved through
the text with a threshold, such that if the information content (e.g., the
entropy, according to Wiener) can be reduced by eliminating a given
character, then that character is eliminated. The threshold is lowered with
each successive paragraph, so that each paragraph becomes more "compressed."

These ideas owe quite a bit to [Allison
Parrish's](https://www.decontextualize.com/) work on using word2vec to
compress texts using DCT (JPEG) compression. In this case, I'm interested in
using probability tables at the level of the letter, which is significantly
simpler, but also very fun to read aloud.

# the resultant text

0

up to this point in this chapter we have been discussing communication systems terminating in machines. in a certain sense, all communication systems terminate in machines, but tordinary language systems terminate in the special sort of machine known as a human being. the human being as a terminal machine has a communication network which may be considered at three distinct levels. for ordinary spoken language, the first human level consists of the ear, and of that part of the cerebral mechanism which is in permanent and rigid connection with the inner ear. this apparatus, when joined to the apparatus of sound vibrations in the air, or their equivalent in electric circuits, represents the machine concerned with the phonetic aspect of language, with sound itself

1

the semantic or second aspect of language is concerned with meaning, and is apparent, for example, in difficulties of translating from olanguage to another where the imperfect correspondence between the meanings of words restricts the flow of information from ointo the other. one may get a remarkable semblance of a language like engliby taking a sequence of words, or pairs of words, or triads of words, according to the statistical frequency with which they occur in tlanguage, and the gibberish thus obtainwill have a remarkably persuasive similarity to good english. this meaningless simulacrum of intelligent speech is practically equivalent to significant language from the phonetic point of view, although it is semantically balderdash, while tengliof an intelligent foreigner whose pronunciation is marked by the country of his birth, or who speaks literary english, will be semantically good and phonetically bad. on the other hand, the average synthetic after-dinner speech is phonetically good and semantically bad.

2

in human communication apparatus, it possible but difficult to determine the characteristics of its phonetic mechanism, and therefore alpossible but difficult to determine what is phonetically significant information, and measure it. it is clear, for example, that the ear and tbrain have an effective frequency cutoff preventing the reception of sohigh frequencies which can penetrate the ear and can be transmitted by the telephone. in other words, these high frequenciewhatevinformatithey may give an appropriate receptor, do not carry any significant amount of information for the ear. but it is even modifficult to determine and measure semantically significant information

3

semantic reception demands memory, and iconsequent long delays. the types of abstractions belonging to timportant semantic stage are nmerely thoassociated with built-in permanentbassemblies of neurons in tbrain, such those which must play a larrole in tperception of geometrical forms; but with abstraction-detector-apparatus consisting parts of the innuncial po- that is, of sets of neurons which aavailable flargassemblies, but are npermanently locked into th- which have betemporarily assemblfor thpurpose.

4

besides thighorganizand maneassemblies in tbrain thundoubtedexist, and afound thoparof tbrain associatwith the organs of specisense, as well as othplaces, there aparticulwitchings and connections whiseem to habeformtemporarily for specipurposesuch learned rexes and tlike. in order form suparticulwitchings, must possible assemble sequences of neurons availabfor tpurpose that are not alreain use. thquestion of assembling concerns, of course, the synaptic thresholds of tsequence neurons assembled. sinneurons exist whican eithbe withoutsiof sutemporaassemblies, it desirable have a speciname for them

5

as i haalreaindicatei consider ththcorrespond rathclosewhat turophysiologists call innuncial pools

6

this leaaasonabof thebehor. themantreceiviaratneithrecentrlates tlangu woword, ba bdea, aoftstimogenerallincertase, it inpositicaon twhopaexperienitrformationathel-tica-overs ana vipart of iwork

7

theisthilevof ccatiowhipresstrlatipfrom tmantlevapfrom tephoneticw. this ttrlation txpencof tindividwhethcciosinctiowhiy observxnallwe mthtbehalevlangu. in tlowanimalit tonlevlawhimobsbeyotticpuactualthis in tcaevebeioththtpiculpershagivpassaressaparticul; in tnththscccess tinnthots othpersonthroutctiotttethectio tp: nldireactionof towhialobsein tlowani; atded aymbsysctiowhiknapokwrittlanguage

8

it thcalnib tof taluchvthfameastmouththntecgeneratltvwieoatithwalat awinmothtissyleaditcaoalntatithgaof tecons, aard tatittxthutted wipdiviystem