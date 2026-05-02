# Trademark Search: OneOS

**Status:** Preliminary search complete; high-risk name  
**Owner:** Project maintainer  
**Last updated:** 2026-05-02

This file records the Phase 0 name search for "OneOS". It is not legal advice
and does not replace review by a qualified trademark attorney.

## Summary

The preliminary search found exact `ONEOS` records in the United States,
including one registration in Class 042 for a cloud-based mobile operating
system service. That is close enough to this project's intended platform/runtime
scope that `OneOS` should be treated as a working codename, not a final public
product brand, until counsel reviews the risk or a replacement name is chosen.

## Scope

Search these sources before Phase 0 exits:

- USPTO trademark database for "OneOS", "One OS", and visually similar marks.
- EUIPO eSearch for the same terms.
- Japan Platform for Patent Information.
- China National Intellectual Property Administration trademark search.
- India Public Search of Trade Marks.
- GitHub, package registries, app stores, and general web search for common-law
  usage in software, operating systems, developer tools, app runtimes, and cloud
  platforms.

## Search log

| Date | Source | Query | Result summary | Link or reference |
|------|--------|-------|----------------|-------------------|
| 2026-05-02 | USPTO Trademark Search | `ONEOS`, `OneOS`, `One OS` | Official web UI is available, but requires JavaScript for interactive searching in this environment. | <https://www.uspto.gov/trademarks/search> |
| 2026-05-02 | USPTO TSDR API | Serial `87269294`, `98814686` | Direct API access requires a USPTO API key; no official status payload retrieved locally. | <https://www.uspto.gov/trademarks/apply/check-status-view-documents/trademark-bulk-data> |
| 2026-05-02 | Markbase USPTO-derived API | `ONEOS` | Returned multiple exact `ONEOS` hits from USPTO-derived data, including serials `87269294`, `97147215`, `98814686`, and historical software/cloud filings. | <https://api.markbase.co/search?q=ONEOS&limit=10> |
| 2026-05-02 | WIPO Global Brand Database | `ONEOS` | WIPO documents the database scope, but direct query access presented an anti-bot challenge in this environment. | <https://www.wipo.int/en/web/global-brand-database> |
| 2026-05-02 | EUIPO API Portal | `ONEOS` | EUIPO exposes a trademark search API, but subscription/sign-in is required. | <https://dev.euipo.europa.eu/product/trademark-search_100> |
| 2026-05-02 | Common-law / web | `oneos.com`, `One OS` | `oneos.com` is an active digital-wallet/wearables site using OneOS / One OS branding. | <https://oneos.com/> |

## Notable matches

| Mark | Jurisdiction/source | Owner | Classes | Status / date seen | Risk note |
|------|---------------------|-------|---------|--------------------|-----------|
| `ONEOS` | USPTO-derived data, serial `87269294`, registration `5461791` | 1APP, Inc | 042 | Registered 2018-05-08; continued-use event shown 2025-01-31 in search results | High. Goods/services describe a cloud-based mobile operating system that runs mobile applications on connected devices. |
| `ONEOS` | USPTO-derived data, serial `97147215` | WEARATEC INC. | 009 | Filing date 2021-11-29; published-for-opposition date 2023-09-05 in Markbase data | Medium to high. Includes downloadable mobile software, digital wallet software, API software, smartwatches, and related devices. |
| `ONEOS` | USPTO-derived data, serial `98814686` | SWVE Management HoldCo, LLC | 045 | Filing date 2024-10-22; response/non-final-action activity shown in search results | Lower for platform/runtime scope, but still an exact active-looking mark. |
| `ONEOS HOME` | USPTO-derived data, serial `98814688` | SWVE Management HoldCo, LLC | 045 | Filing date 2024-10-22 | Lower direct risk, but part of the same exact-word family. |
| `ONEOS` / `One OS` web usage | Common-law/web | Wearatec / One OS site | Payments/wearables | Active site seen 2026-05-02 | Medium. Shows public commercial use of the same wording in technology and mobile-adjacent services. |

## Decision

Do not proceed with `OneOS` as the final product name without attorney review.
For now:

- Treat `OneOS` as a working codename in project materials.
- Do not file a trademark application for `OneOS` during Phase 0.
- Open a naming decision issue before Phase 1 starts.
- Prefer selecting a more distinctive final brand before broad announcement,
  domain purchase, social launch, or external contributor push.

## Follow-up searches

- Complete official USPTO search through the JavaScript UI or with a USPTO API
  key.
- Complete WIPO Global Brand Database search after interactive verification.
- Complete EUIPO search after API/account access or through the public UI.
- Search J-PlatPat, CNIPA, and India Public Search directly.
- Ask counsel whether `OneOS` can remain an internal codename in public docs, or
  whether it should be replaced immediately.
