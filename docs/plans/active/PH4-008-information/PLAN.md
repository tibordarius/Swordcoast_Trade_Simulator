# PH4-008 Plan

1. Add MarketObservationId.
2. Add information module:
   - MarketObservation;
   - KnowledgeView;
   - InformationState.
3. Add RegisterKnowledgeActor and DispatchMarketObservation commands.
4. Capture MarketQuote at dispatch time.
5. Schedule typed information-delivery payload.
6. Deliver observation through AdvanceTo event dispatch.
7. Preserve latest knowledge by observed timestamp, not delivery timestamp.
8. Add freshness/transport-delay query helpers.
9. Add replay/snapshot tests and bump snapshot schema.
10. Run full repository CI.

Compatibility:
- market truth remains unchanged;
- merchant AI is not added;
- no direct global quote access is removed yet, but PH4-010 merchants will consume KnowledgeView.
