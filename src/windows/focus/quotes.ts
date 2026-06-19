// Qur'an verses and hadith about prayer shown in the Focus overlay. One is picked
// at random each time Focus engages.

export interface Scripture {
  text: string;
  source: string;
}

export const SCRIPTURES: Scripture[] = [
  // Qur'an
  {
    text: "Indeed, prayer has been decreed upon the believers a decree of specified times.",
    source: "— Qur'an · An-Nisā 4:103",
  },
  { text: "And seek help through patience and prayer.", source: "— Qur'an · Al-Baqarah 2:45" },
  { text: "Establish prayer for My remembrance.", source: "— Qur'an · Ṭā Hā 20:14" },
  {
    text: "Indeed, prayer restrains from immorality and wrongdoing.",
    source: "— Qur'an · Al-ʿAnkabūt 29:45",
  },
  {
    text: "Successful indeed are the believers — those who are humble in their prayer.",
    source: "— Qur'an · Al-Muʾminūn 23:1–2",
  },
  {
    text: "And establish prayer at the two ends of the day. Indeed, good deeds drive away evil deeds.",
    source: "— Qur'an · Hūd 11:114",
  },
  {
    text: "Guard strictly the prayers, especially the middle prayer, and stand before Allah devoutly obedient.",
    source: "— Qur'an · Al-Baqarah 2:238",
  },
  // Hadith
  { text: "The coolness of my eyes is in prayer.", source: "— Prophet Muhammad ﷺ (an-Nasāʾī)" },
  {
    text: "The closest a servant is to his Lord is while prostrating.",
    source: "— Prophet Muhammad ﷺ (Muslim)",
  },
  { text: "Prayer is light.", source: "— Prophet Muhammad ﷺ (Muslim)" },
  {
    text: "The first deed for which a servant will be held accountable on the Day of Judgment is the prayer.",
    source: "— Prophet Muhammad ﷺ (Tirmidhī)",
  },
];

export const randomScripture = (): Scripture =>
  SCRIPTURES[Math.floor(Math.random() * SCRIPTURES.length)];
