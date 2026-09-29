#!/usr/bin/env python3
"""Sprint 6 integrated validation for the first two-market grain economy."""
from dataclasses import dataclass
from sprint3_pricing import compute_price, market_buy, market_sell, BPS

TICKS_PER_HOUR = 12
TICKS_PER_DAY = 288
MILLI = 1000

class ExactDailyRate:
    def __init__(self, amount_per_day):
        self.amount_per_day=amount_per_day; self.hours=0; self.emitted=0
    def next_hour(self):
        self.hours += 1
        target=(self.amount_per_day*self.hours)//24
        due=target-self.emitted; self.emitted=target; return due

@dataclass
class Market:
    name: str
    stock: int
    target: int
    production_day: int
    demand_day: int
    tier: int
    depth: int
    recent_unmet: int = 0
    total_unmet: int = 0
    min_stock: int = 10**30
    max_price: int = 0
    price_sum: int = 0
    price_samples: int = 0

    def __post_init__(self):
        self.prod_rate=ExactDailyRate(self.production_day)
        self.dem_rate=ExactDailyRate(self.demand_day)

    def hour(self):
        self.stock += self.prod_rate.next_hour()
        request=self.dem_rate.next_hour()
        fulfilled=min(request,self.stock)
        self.stock-=fulfilled
        unmet=request-fulfilled
        self.recent_unmet=unmet
        self.total_unmet+=unmet
        self.min_stock=min(self.min_stock,self.stock)

    def price(self, incoming=0):
        p=compute_price(reference_mcp=2000, available=self.stock, target=self.target, recent_unmet=self.recent_unmet, daily_demand=self.demand_day, effective_incoming=incoming, liquidity_tier=self.tier)
        self.max_price=max(self.max_price,p.fundamental_mcp)
        self.price_sum+=p.fundamental_mcp; self.price_samples+=1
        return p

@dataclass
class Shipment:
    qty: int
    eta: int
    purchase: int
    freight: int

@dataclass
class Result:
    wd_stock: int
    wd_unmet: int
    wd_avg_price: float
    wd_max_price: int
    ath_stock: int
    shipments: int
    realized_profit: int
    capital: int

def simulate(with_merchant: bool, days: int=120, max_in_transit: int=2):
    # Engineering benchmark values, not lore calibration.
    ath=Market('ATHEX',950_000_000,700_000_000,80_000_000,25_000_000,4,300_000_000)
    wd=Market('WDEX',640_000_000,1_200_000_000,10_000_000,45_000_000,5,250_000_000)
    capital=500_000_000
    shipments=[]
    shipment_count=0
    realized_profit=0
    quantity=100_000_000
    freight=31_500_000
    travel_ticks=18*TICKS_PER_DAY
    min_roi_bps=500

    for tick in range(1, days*TICKS_PER_DAY+1):
        if tick % TICKS_PER_HOUR == 0:
            ath.hour(); wd.hour()

        # Arrivals first: cargo becomes physical local stock, then merchant sells ownership.
        arriving=[s for s in shipments if s.eta==tick]
        if arriving:
            for s in arriving:
                dp=wd.price(incoming=0)
                sale=market_sell(quantity_milli=s.qty,bid_mcp=dp.bid_mcp,depth_milli=wd.depth)
                wd.stock += s.qty
                capital += sale['total_mcp']
                profit=sale['total_mcp']-s.purchase-s.freight
                realized_profit += profit
            shipments=[s for s in shipments if s.eta!=tick]

        # Merchant checks every six hours, max two concurrent shipments in v0.
        if with_merchant and tick % (6*TICKS_PER_HOUR)==0 and len(shipments)<max_in_transit:
            incoming=sum(s.qty for s in shipments)
            op=ath.price(0)
            dp=wd.price(incoming//2) # known cargo discounted 50% for ETA/reliability
            buy=market_buy(quantity_milli=quantity,ask_mcp=op.ask_mcp,depth_milli=ath.depth)
            sell=market_sell(quantity_milli=quantity,bid_mcp=dp.bid_mcp,depth_milli=wd.depth)
            deployed=buy['total_mcp']+freight
            profit=sell['total_mcp']-deployed
            roi=(profit*BPS)//deployed if deployed else -999999
            if roi>=min_roi_bps and deployed<=capital and ath.stock>=quantity:
                ath.stock -= quantity
                capital -= deployed
                shipments.append(Shipment(quantity,tick+travel_ticks,buy['total_mcp'],freight))
                shipment_count += 1

        # Daily sample for comparable market stats.
        if tick % TICKS_PER_DAY == 0:
            ath.price(0)
            wd.price(sum(s.qty for s in shipments)//2 if with_merchant else 0)

        assert ath.stock >= 0 and wd.stock >= 0 and capital >= 0

    wd_avg=wd.price_sum/wd.price_samples
    return Result(wd.stock,wd.total_unmet,wd_avg,wd.max_price,ath.stock,shipment_count,realized_profit,capital)

def main():
    control=simulate(False)
    constrained=simulate(True,max_in_transit=2)
    expanded=simulate(True,max_in_transit=8)
    print('without merchant ',control)
    print('2-slot logistics ',constrained)
    print('8-slot logistics ',expanded)
    assert constrained.shipments > 0
    assert constrained.wd_unmet < control.wd_unmet
    assert constrained.wd_avg_price < control.wd_avg_price
    assert expanded.wd_unmet < constrained.wd_unmet
    assert expanded.wd_avg_price < constrained.wd_avg_price
    assert expanded.shipments > constrained.shipments
    assert constrained.capital >= 0 and expanded.capital >= 0
    print('merchant reduces unmet demand: PASS')
    print('merchant reduces average scarcity price: PASS')
    print('higher logistics throughput reduces shortage further: PASS')
    print('route/merchant throughput bottleneck is economically visible: PASS')
    print('no negative stock/capital: PASS')
    print('integrated 120-day vertical slice: PASS')

if __name__=='__main__':
    main()
