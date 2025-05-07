// utils/urlMatcher.ts
type MediaSource = "youtube" | "soundcloud" | null;

const PATTERNS: Record<Exclude<MediaSource, null>, RegExp[]> = {
  youtube: [
    /^(?:https?:\/\/)?(?:www\.)?youtube\.com\/watch\?v=[\w-]{11}(?:[&#?].*)?$/i,
    /^(?:https?:\/\/)?(?:www\.)?youtube\.com\/shorts\/[\w-]{11}(?:[&#?].*)?$/i,
    /^(?:https?:\/\/)?(?:www\.)?youtube\.com\/embed\/[\w-]{11}(?:[&#?].*)?$/i,
    /^(?:https?:\/\/)?youtu\.be\/[\w-]{11}(?:[&#?].*)?$/i,
    // playlist 전체
    /^(?:https?:\/\/)?(?:www\.)?youtube\.com\/playlist\?list=[\w-]+(?:[&#?].*)?$/i,
  ],
  soundcloud: [
    // 트랙  /artist/track
    /^(?:https?:\/\/)?(?:www\.)?soundcloud\.com\/[\w-]+\/[\w-]+(?:\?.*)?$/i,
    // 플레이리스트 /artist/sets/playlist
    /^(?:https?:\/\/)?(?:www\.)?soundcloud\.com\/[\w-]+\/sets\/[\w-]+(?:\?.*)?$/i,
  ],
};

export function match_url(url: string): MediaSource {
  const clean = url.trim();
  for (const [service, list] of Object.entries(PATTERNS) as [
    Exclude<MediaSource, null>,
    RegExp[]
  ][]) {
    if (list.some((re) => re.test(clean))) return service;
  }
  return null;
}
