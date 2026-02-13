# AfriMarketRwa Soroban Platform

AfriMarketRwa ni platform y'ubucuruzi n'ubukode yubatswe kuri Soroban, igizwe n'ibice byinshi (vertical modules) bikoresha **shared core** na **escrow**.

## Uko project yubatswe

- `platform_core/`:
  - Ubwoko (types/enums) busangiwe n'andi ma-contract.
- `escrow/escrow/`:
  - Contract ishinzwe kubika no gusaranganya ubwishyu (`deposit`, `settle_rl`).
- `agriculture/`:
  - Kugurisha umusaruro (items/orders).
- `contracts/rental/`:
  - Ubukode n'igurishwa ry'umutungo.
- `livestock/`:
  - Isoko ry'amatungo, rihuzwa na escrow.
- `health/`:
  - Listing ya services z'ubuzima.
- `transport/`:
  - Listing no kubika (reserve) ride offers.
- `tourism/`:
  - Listing no booking ya tourism packages.

## Icyerekezo cya backend/frontend

### Backend (Soroban contracts)
1. Gukomeza kongeramo tests zambukiranya contracts (escrow + sector modules).
2. Standardization y'events no status transitions ku modules zose.
3. Access-control irushijeho gukomera (admin/provider roles).

### Frontend
1. Kubaka dashboard imwe ifite sections: Agriculture, Rental, Livestock, Health, Transport, Tourism.
2. Gushyiraho API/service layer ihuza contract clients.
3. Gushyiramo flow y'ubwishyu n'igenzura rya transactions (escrow).

## Gutangira

```bash
cargo test --workspace
```

> Niba environment ifite network restrictions, crates download ishobora kubanza kugorana.
