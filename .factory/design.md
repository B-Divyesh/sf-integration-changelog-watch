# Integration Changelog Watch — visual thesis

## Direction: paper-cut operations board

This is a paper-cut diorama of the maintenance work hiding behind an integration: layered vendor notices travel across a desk to an owner, a test, and an acknowledgement. It fits a tool that turns a distant release note into a small, physical next action. The interface is deliberately single-mode light paper rather than a generic SaaS dashboard. A dark environment needs a working surface; this product is that surface.

## Tokens and typography

- Background `#f7f0df` (warm archive paper), surface `#fffaf0`, ink `#192a32`, muted `#52636a`.
- Indigo edge `#284f7a`, amber action `#c95d22`, moss success `#2d6753`, rose warning `#9e3940`.
- Display: Georgia / ui-serif, with cut-paper editorial character. Body: Inter-like system sans (`system-ui`), self-hosted by the platform, so no remote font request is made.
- Spacing uses an 8px rhythm. Panels have clipped-corner shapes and thin shadow layers which imitate stacked card stock.

## Interaction and motion

The important motion is a notice card sliding one short step from the feed shelf to the action stack when a scan finds a match (220ms transform/opacity). Hover lifts paper only 2px. `prefers-reduced-motion` removes both transforms and replaces them with an instant outline change. Focus rings are a high-contrast indigo dashed stitch.

## Asset prompt sheet and provenance

Hero art is an original generated paper-cut illustration: a layered cream paper desk, three unlabeled release-note cards moving along an indigo thread into a small amber action card; tactile cut edges, editorial top-down diorama, warm archive paper / deep indigo / muted moss / amber palette, no people, no brands, no letters, no logos, no watermark. It is used as a supporting image, never as text. Generated through the factory image workflow on 2026-08-28; original factory-generated asset. The optimized WebP stays below 300 KB.

## Accessibility and responsive intent

At phone width, the scene becomes a narrow supporting strip and the watch list stacks before action cards. Text never sits in the image. All status also has words; color is only reinforcement. Paper layers are decoration and are removed from the accessibility tree.
