Story

Idée générale :
- schyzo policier
- train / wagon recherche
le policier ne sait rien et essaie d'avancer dans sa quête de savoir ce qu'il se passe (il essaie d'établir des liens entre les choses qu'il écrit au fur et à mesure dans son carnet de notes) *(en quête d'enquête)*.
Le policier cherche ses médicaments car il a mal à la tête, il demande aux autres personnes qui leur racontent des anecdotes/parties d'histoire qui sont à l'opposé de ce qu'il veut savoir et se retrouve impliqué dans un mystère qu'il essaie de résoudre
- crise/hallucination à intervalles réguliers (limite de temps => au reset il garde track de ses relations (par rapport à l'arbre) (indice ce n'est pas vraiment une hallucination) et son but est d'avancer plus rapidement et plus intelligement par rapport aux dialogues)
Les dialogues se débloquent en fonction de son avancée (dans l'arbre)
- Les NPC ont des routines (déplacements, (changements))
- Indices sur des objets et interactions par rapport aux salles

Personnages:
A chaque discussion avec un personnage le carnet de notes s'enrichit plus ou moins pertinemment en fonctionde l'avancée déjà présente dans le carnet. (La meta veut qu'il ne faut pas aller discuter tout le temps et à tout le monde directement sinon les informations peuvent être contradictoires qui ne pourront être fixées que par une information qui éclaire la situation et sera donc apportée plus tard).
Quand le déplacement d'un personnage est aléatoire (sauf conducteur) cela concerne tous les wagons sauf celui du conducteur.

- Policier (main):
ex policier, vient de se faire virer à cause de problèmes mentaux, son collègue l'accompagne pour le "destituer" et genre le surveiller vite fait.

- L'allumeuse:
    Caractère :
    - Elle est complètement folle, elle donne pour la plupart du temps des informations fausses. Elle ne cherche qu'à te faire perdre ton temps dans de longs dialogues.
    Gameplay:
    - Elle fait de longs dialogues difficilement skipable.
    Routine :
    - Se déplace très peu entre des wagons random.
    Dialogues :
    - (default) -> "Eh beau gosse tu m'invites?" (default) -> "T'as de quoi m'inviter oui ou non?" (default) -> "..."/"Si t'es fauché dis le tocard!"
    -> (loop)
    (parlé racaille) -> "..."/"Adultère de quoi?"/"Et qu'est-ce que t'en sais hein? T'as aucune preuve de ce que t'avances!"/"Déguerpis d'ici tocard!" -> (loop)
    (parlé barista) -> "..."/"..."/"Elle a lâché l'affaire..?"/"Cette barista m'a enfin lâchée..."/
    "Laisser ton ami policier tranquille? ..."/
    "C'est toujours pratique d'avoir un policier à sa botte."/
    "... Ne crois pas que je vais le lâcher pour si peu, celui-là n'avait qu'à pas tromper sa compagne!"
    (continue) -> "... Ok... Je veux peut-être bien le laisser tranquille si tu m'offres quelque chose d'équivalent."/
    "Un verre? Haha tu penses que ce sera assez?"/
    "Des dizaines de verres?"/
    "..."/
    "Mon mignon tu sais parler aux femmes."
    -> (loop) (event : barista disparaît et le controlleur est au bar)

- Les vieux:
    Caractère :
    - Se font chier et ils sont seuls (ils sont vieux) du coup ils sont trop heureux si tu viens leur parler 
    Gameplay:
    - Ils te donnent des bonbons.
    Routine :
    - ne se déplacent pas mais ils te donnent plein d'infos, ils te donnent les bonbons qui te permettent de parler aux gamins
    Dialogues :
    - (default) -> "Bonjour jeune homme!" -> (continue) "Nous passons un très bon voyage, nous allons voir nos petits enfants."/
    "Passez-vous un bon voyage?" -> (loop)
        (loop) -> "L'assassinat du président X est vraiment terrible. Si ces satanés complotistes n'avaient pas pris cette vie, nous aurons été tous bien plus heureux!"
        (loop) -> "Le conducteur a sûrement plein d'histoires à raconter!"
    (parlé aux gosses) -> "Vous n'avez pas bonne mine jeune homme!"/
    "..."/
    "Tenez prenez ces friandises."/
    "Avec cela vous serez de bonne humeur à coup sûr!" -> (loop)

- Average man (duplicable):
    Caractère :
    - Il est présent physiquement. Il ne semble pas très enjoué et peut donner quelques informations selon une probabilité ou l'avancée du policier.
    Gameplay:
    - Ils peuvent te donner des choses?
    - Le vol est possible (similaire à Average woman).
    Routine :
    - Fixe.
    Dialogues :
    - (default) -> "Bonjour monsieur." (continue) -> "Avez-vous vu ces enfants? C'est terrible... Ils ont perdu leurs parents."/
    "Comment? Apparemment ils ont été pris dans un accident. Cependant..."/
    "Cependant les enfants ne semblent bizarrement pas si tristes. Leurs parents étaient-ils des monstres?"
    (continue) -> "Je n'ai jamais compris ce genre de parents tyranniques."/
    "Il faudrait peut être aider ces enfants."
    (continue) -> (loop)

- Average woman(duplicable):
    Caractère :
    - Elle est présente physiquement. Elle ne semble pas très enjouée et peut donner quelques informations selon une probabilité ou l'avancée du policier.
    Gameplay:
    - Elles peuvent te donner des choses?
    - Des bijoux peuvent déborder de son sac qui donne une information pour un vol potentiel. (L'information est notée et révélée dans le carnet de notes si l'avancée du policier nécessite de faire intervenir un vol (s'il a parlé à la racaille avant)).
    Routine :
    - Fixe.
    Dialogues :
    - (default) -> "Bonjour... Je peux vous aider?"
      (parlé à ton pote policier et controlleur) -> "C'est terrible ce qui s'est passé avec l'assassinat du président X..."/
      "Comment ça ça s'est passé quand? En 1965 enfin!
      Ces abrutis soupçonnaient un complot avec notre cher président, c'est n'importe quoi!"
      (continue) -> "Le président X était vraiment admirable! Il était bienveillant et aimé de tous."/
      "Comment ça qui étaient ses assassins? Vous habitez dans une grotte?"/
      "On soupçonnait une organisation derrière tout ça... Ils auraient cru à des manigances de notre cher président, c'est totalement absurde!" (continue) -> (loop)
      (parlé racaille) -> "Cette saleté de racaille me lance des regards lugubres parfois."

- L'autre policier (rationnel):
    Caractère :
    - Plutôt raisonné, semble faire attention à son ami policier et lui rappelle constamment qu'il est en délire qu'il divague mais au bout d'un moment peut confirmer ou non des informations qui sont potentiellement erronées.
    Gameplay:
    - Ils peuvent te donner des choses?
    Routine :
    - Fixe.
    Dialogues :
    - (default) -> "..."/"Mon pote."/
    "Arrête d'avoir cette mine!"/
    "..."/
    "Écoute, je sais que ton licenciement t'a grandement bouleversé mais resaisis-toi!"/
    "Il y a un psy' à bord du train, cherche le et profites-en pour prendre l'air."
    (parlé psy) -> "Alors comment ça s'est passé?"/
    "Ré-réhabilitation? Enquête..? Il t'a carrément remis le boulot en tête celui-là!"/
    "..."/
    "Bon..."/
    "J'ai compris! Si tu insistes autant je peux te donner une chance."/
    "Quelque chose se trame dans ce train, il a été objet de quelques enquêtes, en effet des personnes ont été portées disparues."/
    "... Ah non c'est rien. Je me dis juste que tu n'avais pas à avoir cette enquête... Je suis désolé."/
    "Moi? Pourquoi je ne mène pas l'enquête? Mais voyons tu ne vois pas que je suis justement dans le train avec toi!"/
    "Je- je... Je suis juste en standby! Oui c'est ça! En standby!"/
    "J'étudie la scène, tu sais en analysant les objets et en interrogeant les passagers. Tu devrais faire de même."/
    "En tout cas, reviens me voir si tu as une soumission à me faire par rapport à l'enquête. Je t'épaulerai comme il faut, mais ne te surmène pas, après tout tu es..."/
    "Enfin, bon courage." -> (loop)

- Le psy/médecin (conseiller/tuto):
    Caractère :
    - Il te conseille sur le gameplay (sans briser le quatrième mur) (ex: note des informations dans ton carnet comme ça tu t'en souviens) et peut te donner du temps supplémentaire (des médocs).
    Gameplay:
    - Il peut te donner des médicaments qui augmente le temps restant.
    Routine :
    - Fixe.
    Dialogues :
    - (default) -> "Bonjour monsieur."/
    "Vous me semblez un peu perdu."/
    "..."/"Ah! Mais vous êtes ce fameux inspecteur!"/
    "Ou du moins ce qu'il en reste..."/
    "Hum. Excusez-moi, je suis psychologue de profession."/
    "Votre situation actuelle me semble quelque peu emmêlée, laissez-moi vous éclairer."/
    "Hmm... Votre réhabilitation est en elle-même une enquête..."/
    "Vous êtes dans un sacré cas!"/
    "Étant donné votre situation actuelle vous pouvez noter dans votre carnet toute information qui vous semble bon à prendre."/"Prenez toutefois garde, vos phases peuvent vous jouer des tours quant à la véracité de vos notes."/"Pensez donc à consulter régulièrement votre entourage pour vous rappeler d'une certaine cohérence."/
    "..."/
    "Cette enquête semble être potentiellement la plus dure de celle de toute votre carrière!"/
    "Mais bon... Je crois en vous.
    Vous étiez tout de même le meilleur inspecteur du pays!"/
    "Bon courage et que le chemin de la réhabilitation vous sourit."
    -> (loop)
    (parlé controlleur) -> "Ah monsieur!"/"Vous semblez mal au point, ou me trompe-je?"/"Voici quelques médicaments..." -> (loop)

- Les gosses:
    Caractère :
    - Sont cons donc si t'as pas les bonbons ils te mentent, sinon ils te disent la vérité sur des informations potentiellement pertinentes (des ragots).
    Gameplay:
    - Rien.
    Routine :
    - se déplacent dans tous les wagons, ils ne sont pas tout le temps présent.
    Dialogues :
    - (default) -> "Tema la taille du rat!"/"Quoi?"/"Quoicoubeh!" -> (loop)
      (bonbons et parlé à average man) -> "Oh des bonbons!! Merci monsieur!"
      (continue) -> "Nos parents? Ils étaient méchants et se fichaient de nous. M. le controlleur et M. le conducteur s'occupent de nous maintenant et c'est bien mieux!!"/
      "On voyage maintenant dans le train avec eux."
      -> "On s'amuse bien ici mais les gens partent aussi vite qu'ils arrivent."
      -> "Que sont devenus nos parents exactement? On a pas trop compris mais ils sont descendus du train sans nous non?"
      (continue) -> "Merci encore sur les bonbons!!" -> (loop)
    (parlé conducteur) -> "M. le conducteur est très gentil avec nous!!"/"Il dit des choses un peu bizarres parfois mais il est sympa."

- La barista:
    Caractère :
    - Elle connaît les habitudes des gens mais il faut réussir à la faire parler, elle est plutôt froide et distante.
    Gameplay:
    - Rien.
    Routine :
    - Fixe.
    Dialogues :
    - (default) -> "Bonjour..." (continue) -> "Vous êtes bien monsieur Y...?"/
    "Votre réputation semble vous précéder. J'espère que votre voyage se passe bien..."/
    "Quelle réputation? Mais... Vous êtes le meilleur inspecteur du pays!"/
    "Enfin... vous étiez."
    (continue) -> "Que puis-je vous servir?" -> (loop)
    (parlé à average man/average woman) -> "Je-je n'en sais pas plus sur eux que sur vous vous savez."/
    "Si- si je vous l'assure!"/"..."/"Prendrez-vous un verre?" -> (loop)
    (parlé à l'autre policier) -> "Votre ami policier semble bien vous apprécier."/
    "Nou- je l'- l'apprécie tout autant enfin!"/
    "..."/
    "Mais si vous voulez mon avis, il ne semble pas très net..." -> (loop)
    (parlé à l'allumeuse de l'adultère) "Bonjour."/"Une adultère..?"/
    "... On vous a donc mis au courant."/
    "C'est cette saleté de racaille..!"/
    "Je-je..."/
    "Que je me calme? C'est- je n'ai rien à voir avec cette histoire..!"/
    "Mais enfin... C'est- c'est moi qui suis victime!"/
    "Puisque je vous dis que je suis coupabl- ... victime!!!"/
    "..."/
    "..."/
    "..."/
    "Désolée... Vous- vous devez comprendre que cett- cette histoire me- nous dépasse largement."/
    "Je peux tout vous expliquer si vous me promettez de m'aider à me sortir de là."/
    "..."/
    "On m'a simplement dit que votre ami le policier ne devait pas mener d'enquête sur quoi que ce soit dans ce train."/
    "On? ..."/
    "Le controlleur..."/
    "Chhh! Ne parlez pas si fort!"/
    "Il regarde tout et entend tout partout."/
    "J'ai donc convaincu cett- cette allumeuse qui s'ennuyait qu'elle commette cette adultère avec lui..."/
    "Il ne lui a fallu que quelques verres."/
    "Et maintenant, elle le tient en laisse."/
    "Que se passe-t-il ensuite? C'est à vous de me dire! Je- je vous ai dit tout ce que je savais, aidez-moi maintenant!" -> (loop)

- Le conducteur:
    Caractère :
    - Il connaît la structure du train (comment sont agencés les wagons) et donc à des horaires précis connaît l'état des salles.
    Il est sympa mais semble ignorant.
    Gameplay:
    - Rien.
    Routine :
    - Fixe.
    Dialogues :
    - (default) -> "Bien le bonjour et bienvenue à bord monsieur!"/"Notre train est réputé pour toujours arriver à bonne destination. Donc n'ayez crainte pour votre voyage."
    (parlé l'autre policier) -> "Ce policier... Il m'a semblé un peu crispé par la situation actuelle. Je pense qu'il doit être bloqué par quelque chose."/
    "En parlant de bloquer, la porte du wagon de cargaisons était bloqué toute la veille!"/
    -> (loop)
    (parlé barista) -> "La barista sert de très bons cocktails."/
    "... Non pas que j'en prenne au volant!"
    (continue) -> "Pourquoi semble-t-elle un peu stressée vous dites?"/
    "..."/
    "Elle doit peut être avoir quelques complications dans sa vie."/
    "Mais... Je crois qu'elle est sur les rails de la réhabilitation." -> (loop)
    (parlé controlleur) -> "Les ragots peuvent parfois toujours être source de conflits n'est-ce pas? Ne vous embêtez pas avec de telles sottises." -> (loop)
    (parlé racaille) -> "Cette racaille... Elle occupe le wagon de cargaison presque tout le temps!" -> (loop)
    (parlé gosses) -> "Ah ces enfants?"/
    "Ils ont en effet perdu leurs parents dans un tragique accident."/
    "Que ce soit pour le meilleur ou le pire ces enfants semblent maintenant libérés!"/
    "Leur éducation était à revoir, les parents les avaient délaissés. Les pauvres." (continue) -> "Ces enfants sont comme nos enfants maintenant!"/"Nos? Haha! Je veux dire ce sont les enfants du train maintenant!" -> (loop)
    (parlé average man) -> "Cet homme lambda ne semble pas au plus haut point dans sa vie."/
    "Peut-être aurait-il besoin d'un peu d'aide." -> (loop)

- Le controlleur:
    Caractère :
    - Il faut lui parler plusieurs fois à plusieurs moments pour qu'il donne des informations cruciales sur les personnes qu'il a récoltées à travers des contrôles. Il est un peu lourd parfois donc il force les gens à divulger leurs informations pour qu'ils soient en règle.
    Gameplay:
    - Rien de spécial.
    Routine :
    - Bouge assez peu, il alterne entre le bar et les derniers wagons.
    Dialogues :
    - (default) -> "..."/"Bonjour monsieur."
      (continue) -> "..."/"Vous semblez un peu malheureux."/"Si jamais vous l'êtes vraiment, vous pouvez peut-être tenter d'être heureux en profitant de ceux qui le sont." -> (loop)
      (parlé vieux) -> "..."/"N'écoutez pas trop ce que peuvent dire les autres, les ragots sont toujours présents de nos jours et croyez-moi qu'ils ne vous rendront pas votre vie toute rose." -> (loop)
      (parlé allumeuse) -> "Vous..."/"Où allez-vous?"/"..."/
      "Monsieur. Allez-vous bien? Vous me paraissez un peu pâle."/
      "Oui tout à fait vous ne semblez pas avoir bonne mine."/
      "..."/
      "Monsieur, j'insiste. Je crois que vos problèmes de santé vous reprennent et j'ai peur que cela n'empire immédiatement. Par chance il y a un médecin à bord, je vous y emmène directement."/
      "Allons-y."
      -> (TP wagon)

- La racaille:
    Caractère :
    - Il cherche les problèmes et si le policier l'aborde trop il le frappe et il tombe dans le coma. Si le controlleur est présent dans le wagon alors il ne fera rien et donnera potentiellement des informations contre des informations (il souhaite voler d'autres personnes => dépend de notre avancée).
    Gameplay:
    - Il permet de time reset.
    Routine :
    - Fixe.
    Dialogues :
    - "Qu'est-ce tu veux frère?"
    (default) -> te pète la gueule -> back to init
    (besoin information adultère/pote) -> "Ouais je peux te donner des infos mais donne moi un truc qui vaut le coup en échange."
    (parlé à average man/woman) -> "Ouais c'est pas mal ça! Ces pouilleux n'avaient qu'à bien ranger leurs affaires!" -> "..."/"Ah ouais. Tiens tu sais quoi? J'ai pris en flagrant délit ton pote l'policier avec l'allumeuse du coin, ils faisaient quelque chose de pas très catho' s'tu vois c'que je veux dire haha!"/
    "Ce que je sais de plus? Hmm... J'crois que l'allumeuse le garde sous le coude mais on dirait qu'elle souhaite pas en dire plus que ça."/
    "Pfff... C'est une allumeuse non? Elle doit avoir l'habitude de faire ça avec tout le monde donc pourquoi faire la cachottière?" -> (back to init)
    (parlé à l'autre policier et photo adultère) -> "..."/"Et qu'est-ce que tu veux que j'y fasse dans ton histoire?"/"Hein?! Un trésor? Sérieux?"/
    "..."/
    "Ok tout ce dont t'as besoin c'est d'un gros dur comme moi? Je suis avec toi!" -> (loop)

Objectif : 
Arriver à 100% de découverte du mystère, pourcentage défini par ta progression dans le graphe des connaissances.
Le policier progresse en parlant aux NPC et aux interactions qui complètent son carnet de notes (qui contient aussi l'arbre avec les relations),
sa progression est conservée tout le long des time reset (coma).

Le mystère en question :
Le conducteur et le controlleur sont en fait membres d'une secte, ils essaient de recruter des gens. Le controlleur a l'air froid et désagréable parce que personne lui convient, et c'est pour ça qu'il a l'air plus gentil avec nous => il sent qu'on est potentiellement recrutable. Ceux qui sont pas recrutables euuuuh force à eux (spoiler ils meurent).

Indices vers le mystère : 
- La salle de la chaudière est fermée, et askip ça sent mauvais dans le coin (TODO : qui donne cette info ?)
- Le controlleur demande si on a de la famille / des connaissances qui nous attendent à l'arrivée (pour savoir si on est isolé, donc plus facilement recrutable) 
- Peut être la barista qui nous laisse penser qu'il y a une machinerie et qu'elle souhaite nous aider à condition qu'on l'aide à s'en sortir.
- Les gosses étaient en voyage avec leurs parents, qui se sont fait ratio+palu (parce qu'ils étaient trop heureux cf Secte). Ils le savent pas, pour eux le conducteur et le controlleur les ont sauvé de parents "méchants"/qui s'occupent pas bien d'eux, et maintenant ils restent dans le train et ils voyagent avec. Ils te disent que la raison donnée par le conducteur qui les a recueilli (leur sauveur) est que les parents ne pensaient qu'à eux et ne voulaient pas le bonheur de leurs enfants.
- La secte originale souhaitait rendre heureux leurs adeptes qui sont des personnes malheureuses. Toutefois le défunt co-chef a changé le chef qui a fait dériver l'objectif de la secte en si les adeptes sont malheureux c'est à cause des personnes heureuses donc il faut les tuer.
Le chef de la secte originale était bienveillant et ami du président du pays, cependant ses activités et motivations ont été grandement aidées par le président qui a porté le rôle de co-chef, il a été repéré par des organisations qui ont souhaité l'assassiner en raison d'une peur de déviation de motivations en secte.
L'autre policier est au courant de telles affaires et souhaitait mener son enquête, il est dans le train car il a mené son enquête qui l'a mené dans ce train. Cependant, il a été pris en pleine adultère avec l'allumeuse par la racaille, la barista blackmail l'allumeuse par la secte pour qu'elle bloque l'autre policier.
La tâche de la barista est également de retenir le plus possible le main car il était connu pour être le meilleur enquêteur (il a été viré en raison de son accident traumatique qui a été causé par la secte).
Le main poursuit une quête de bonheur qui est en fait sa véritable quête d'origine, retrouver son bonheur perdu à cause de son accident traumatique. C'est pour cela qu'il essaie inconsciemment d'élucider le mystère du train qui s'avère être lié à son accident traumatique, son propre mystère, son bonheur, son enquête...

Déroulé:
    Le fil de la véritable histoire consiste en aller parler à tous les passagers pour récolter assez d'informations et débloquer mutuellement les dialogues et compléter le carnet. Le vecteur directeur est la barista qui tient en laisse l'allumeuse qui tient elle-même en laisse l'autre policier. Débloquer cela permet de convaincre partiellement l'ami policier d'aller aider le main,
    cependant il faut lui donner la photo de l'adultère lâchée dans le bar après que la barista ait disparu.
    L'autre policier hésitera toujours et il faudra aller parler et faire une fausse promesse à la racaille qui jouera de gros bras supplémentaire et finir par une soumission à l'autre policier pour débusquer la secte.