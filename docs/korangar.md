# Referencia de Packets ZC — Korangar Client

**Servidor**: AnimaRO (rAthena)
**PACKETVER**: `20220406` (`PACKETVER_MAIN_NUM`, NO RE, NO ZERO)
**Fecha**: 2026-02-08

---

## Configuración del Servidor

El servidor compila con:

```c
#define PACKETVER 20220406
```

**Condicionales activos**:
- `PACKETVER_MAIN_NUM = 20220406` → checks `PACKETVER_MAIN_NUM >= X` donde X ≤ 20220406 son **TRUE**
- `PACKETVER_RE_NUM` → **no definido**, todos los checks RE son **FALSE**
- `PACKETVER_ZERO_NUM` → **no definido**

**Consecuencias globales** (afectan múltiples packets):
- Item IDs son `u32` (no `u16`) — MAIN_NUM >= 20181121
- Card slots son `u32[4]` = 16 bytes — MAIN_NUM >= 20181121
- Equipment location es `u32` — PACKETVER >= 20120925
- Weapon/Shield en spawn son `u32` — >= 20181121
- `ItemOption` = `{index: i16, value: i16, param: u8}` × 5 = 25 bytes

---

## Tipos Globales

```
EquipSlotInfo    = u32[4]                           (16 bytes)
ItemOption       = { index: i16, value: i16, param: u8 }  (5 bytes)
ItemOptions      = ItemOption[5]                    (25 bytes)
PosDir           = u8[3]    — X(10b) + Y(10b) + Dir(4b)
MoveData         = u8[6]    — fromXY + toXY packed
MapName          = u8[16]   — null-terminated
CharName         = u8[24]   — null-terminated
```

---

## 1. Conexión y Autenticación

### 1.1 ZC_ACCEPT_ENTER — `0x0A18` — 16 bytes

| Offset | Tipo | Campo | Nota |
|--------|------|-------|------|
| 0 | u16 | header | `0x0A18` |
| 2 | u32 | start_time | server tick |
| 6 | u8[3] | pos_dir | posición + dirección packed |
| 9 | u8 | x_size | siempre 5 |
| 10 | u8 | y_size | siempre 5 |
| 11 | u16 | font | |
| 13 | u8 | sex | 0=F, 1=M |

### 1.2 ZC_REFUSE_ENTER — `0x0074` — 3 bytes

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | u8 | error_code |

### 1.3 ZC_RESTART_ACK — `0x00B3` — 3 bytes

| Offset | Tipo | Campo | Nota |
|--------|------|-------|------|
| 0 | u16 | header | |
| 2 | u8 | type_ | 0=puede respawnear, 1=no |

### 1.4 ZC_NPCACK_MAPMOVE — `0x0091` — 22 bytes

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | u8[16] | map_name |
| 18 | u16 | x |
| 20 | u16 | y |

### 1.5 ZC_NOTIFY_VANISH — `0x0080` — 7 bytes

| Offset | Tipo | Campo | Nota |
|--------|------|-------|------|
| 0 | u16 | header | |
| 2 | u32 | gid | entity ID |
| 6 | u8 | type_ | 0=out of sight, 1=dead, 2=logout, 3=teleport, 4=trickdead |

---

## 2. Unit Spawn (Packets Más Complejos)

### 2.1 ZC_NOTIFY_NEWENTRY (Spawn) — `0x09FE` — Variable

| Offset | Tipo | Campo | Nota |
|--------|------|-------|------|
| 0 | u16 | header | `0x09FE` |
| 2 | u16 | packet_length | |
| 4 | u8 | object_type | 0=PC, 1=NPC, 5=MOB, 4=HOMUN, 6=MERC, 7=ELEM |
| 5 | u32 | aid | account ID |
| 9 | u32 | gid | char ID |
| 13 | i16 | speed | |
| 15 | i16 | body_state | opt1 |
| 17 | i16 | health_state | opt2 |
| 19 | i32 | effect_state | option |
| 23 | i16 | job | class ID |
| 25 | u16 | head | hair style |
| 27 | u32 | weapon | **u32** |
| 31 | u32 | shield | **u32** |
| 35 | u16 | accessory | head bottom |
| 37 | u16 | accessory2 | head top |
| 39 | u16 | accessory3 | head mid |
| 41 | i16 | head_palette | hair color |
| 43 | i16 | body_palette | body color |
| 45 | i16 | head_dir | face direction |
| 47 | u16 | robe | garment ID |
| 49 | u32 | guild_id | |
| 53 | i16 | guild_emblem_ver | |
| 55 | i16 | honor | manner |
| 57 | i32 | virtue | |
| 61 | u8 | is_pk_mode | |
| 62 | u8 | sex | 0=F, 1=M, 2=unknown |
| 63 | u8[3] | pos_dir | posición + dirección |
| 66 | u8 | x_size | |
| 67 | u8 | y_size | |
| 68 | i16 | clevel | |
| 70 | i16 | font | |
| 72 | i32 | max_hp | mobs con HP visible |
| 76 | i32 | hp | |
| 80 | u8 | is_boss | 0=normal, 1=boss, 2=mini |
| 81 | i16 | body | body override |
| 83 | u8[24] | name | |

### 2.2 ZC_NOTIFY_MOVEENTRY (Moving) — `0x09FD` — Variable

Igual que `0x09FE` pero:
- Offset 63: `u8[6] move_data` (from→to) en lugar de `u8[3] pos_dir`
- Los offsets posteriores se desplazan +3 bytes

### 2.3 ZC_NOTIFY_STANDENTRY (Idle) — `0x09FF` — Variable

Igual que `0x09FE` pero:
- Después de `y_size`: campo extra `u8 state`
- Los offsets posteriores se desplazan +1 byte

---

## 3. Movimiento

### 3.1 ZC_NOTIFY_PLAYERMOVE — `0x0087` — 12 bytes

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | u32 | move_start_time |
| 6 | u8[6] | move_data |

### 3.2 ZC_STOPMOVE — `0x0088` — 10 bytes

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | u32 | aid |
| 6 | u16 | x |
| 8 | u16 | y |

### 3.3 ZC_CHANGE_DIRECTION — `0x009C` — 9 bytes

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | u32 | src_id |
| 6 | u16 | head_dir |
| 8 | u8 | dir |

---

## 4. Stats y Parámetros

### 4.1 ZC_STATUS — `0x00BD` — 44 bytes

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | u16 | status_points |
| 4 | u8 | str |
| 5 | u8 | str_needed |
| 6 | u8 | agi |
| 7 | u8 | agi_needed |
| 8 | u8 | vit |
| 9 | u8 | vit_needed |
| 10 | u8 | int |
| 11 | u8 | int_needed |
| 12 | u8 | dex |
| 13 | u8 | dex_needed |
| 14 | u8 | luk |
| 15 | u8 | luk_needed |
| 16 | i16 | atk |
| 18 | i16 | atk2 |
| 20 | i16 | matk_max |
| 22 | i16 | matk_min |
| 24 | i16 | def |
| 26 | i16 | def2 |
| 28 | i16 | mdef |
| 30 | i16 | mdef2 |
| 32 | i16 | hit |
| 34 | i16 | flee |
| 36 | i16 | flee2 |
| 38 | i16 | crit |
| 40 | i16 | aspd |
| 42 | i16 | aspd2 |

### 4.2 ZC_PAR_CHANGE — `0x00B0` — 8 bytes

| Offset | Tipo | Campo | Nota |
|--------|------|-------|------|
| 0 | u16 | header | |
| 2 | u16 | var_id | SP_* enum |
| 4 | i32 | count | nuevo valor |

**SP_* values comunes**: SP_SPEED(0), SP_BASELEVEL(5), SP_JOBLEVEL(55), SP_STATUSPOINT(9), SP_SKILLPOINT(251), SP_HP(5), SP_MAXHP(6), SP_SP(7), SP_MAXSP(8), SP_WEIGHT(24), SP_MAXWEIGHT(25), SP_ATK1(41), SP_DEF1(42), SP_MDEF1(43), SP_ATK2(44), SP_DEF2(45), SP_MDEF2(46), SP_HIT(17), SP_FLEE1(18), SP_FLEE2(19), SP_ASPD(53), SP_ATTACKRANGE(1000)

### 4.3 ZC_LONGPAR_CHANGE — `0x00B1` — 8 bytes

| Offset | Tipo | Campo | Nota |
|--------|------|-------|------|
| 0 | u16 | header | |
| 2 | u16 | var_id | SP_BASEEXP, SP_JOBEXP, SP_NEXTBASEEXP, SP_NEXTJOBEXP, SP_ZENY |
| 4 | i32 | amount | |

### 4.4 ZC_STATUS_CHANGE_ACK — `0x00BC` — 6 bytes

| Offset | Tipo | Campo | Nota |
|--------|------|-------|------|
| 0 | u16 | header | |
| 2 | u16 | sp | stat param ID |
| 4 | u8 | ok | 0=fail, 1=success |
| 5 | u8 | value | nuevo valor |

### 4.5 ZC_COUPLESTATUS — `0x0141` — 14 bytes

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | u32 | status_type |
| 6 | i32 | default_status |
| 10 | i32 | plus_status |

### 4.6 ZC_NOTIFY_EXP — `0x0ACC` — 18 bytes

| Offset | Tipo | Campo | Nota |
|--------|------|-------|------|
| 0 | u16 | header | |
| 2 | u32 | account_id | |
| 6 | **i64** | amount | **8 bytes!** |
| 14 | u16 | var_id | SP_BASEEXP o SP_JOBEXP |
| 16 | u16 | exp_type | 0=normal, 1=quest |

---

## 5. Combate y Daño

### 5.1 ZC_NOTIFY_ACT — `0x08C8` — 31 bytes

| Offset | Tipo | Campo | Nota |
|--------|------|-------|------|
| 0 | u16 | header | |
| 2 | i32 | src_id | atacante |
| 6 | i32 | target_id | víctima |
| 10 | i32 | server_tick | |
| 14 | i32 | src_speed | anim speed atacante |
| 18 | i32 | dmg_speed | anim speed daño |
| 22 | i32 | damage | daño principal |
| 26 | i8 | is_sp_damage | |
| 27 | u16 | div | nro de hits |
| 29 | u8 | type_ | 0=dmg, 1=pickup, 2=sit, 3=stand... |
| 30 | i32 | damage2 | dual-wield |

**Tipos de acción**: 0=Damage, 1=Pickup, 2=Sit, 3=Stand, 4=EndureDamage, 5=Splash, 6=Skill, 7=Repeat, 8=MultiHit, 9=MultiHitEndure, 10=Critical, 11=LuckyDodge, 12=TouchSkill, 13=MultiHitCritical

### 5.2 ZC_ATTACK_RANGE — `0x013A` — 4 bytes

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | i16 | range |

### 5.3 ZC_ATTACK_FAILURE_FOR_DISTANCE — `0x0139` — 16 bytes

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | u32 | target_aid |
| 6 | i16 | target_x |
| 8 | i16 | target_y |
| 10 | i16 | x (atacante) |
| 12 | i16 | y (atacante) |
| 14 | i16 | range |

### 5.4 ZC_ACTION_FAILURE — `0x013B` — 4 bytes

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | u16 | type_ |

---

## 6. Status Effects / Buffs

### 6.1 ZC_MSG_STATE_CHANGE — `0x0196` — 9 bytes

| Offset | Tipo | Campo | Nota |
|--------|------|-------|------|
| 0 | u16 | header | |
| 2 | u16 | index | status index |
| 4 | u32 | id | entity ID |
| 8 | u8 | state | 1=on, 0=off |

### 6.2 ZC_MSG_STATE_CHANGE2 — `0x043F` — 25 bytes

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | u16 | index |
| 4 | u32 | id |
| 8 | u8 | state |
| 9 | u32 | remain_msec |
| 13 | u32 | val1 |
| 17 | u32 | val2 |
| 21 | u32 | val3 |

### 6.3 ZC_MSG_STATE_CHANGE3 — `0x0983` — 29 bytes

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | u16 | index |
| 4 | u32 | id |
| 8 | u8 | state |
| 9 | u32 | total_msec |
| 13 | u32 | remain_msec |
| 17 | u32 | val1 |
| 21 | u32 | val2 |
| 25 | u32 | val3 |

---

## 7. Inventario

### 7.1 ZC_ITEM_PICKUP_ACK — `0x0B41` — ~70 bytes

| Offset | Tipo | Campo | Nota |
|--------|------|-------|------|
| 0 | u16 | header | |
| 2 | u16 | index | |
| 4 | u16 | count | |
| 6 | **u32** | item_id | **u32!** |
| 10 | u8 | identified | |
| 11 | u8 | damaged | |
| 12 | **u32[4]** | cards | **16 bytes, u32 cada slot** |
| 28 | u32 | location | |
| 32 | u8 | type_ | |
| 33 | u8 | result | 0=ok, 1=inv full, 2=overweight |
| 34 | i32 | hire_expire | |
| 38 | u16 | bind_on_equip | |
| 40 | ItemOption[5] | options | 25 bytes |
| 65 | u8 | favorite | |
| 66 | u16 | look | |
| 68 | u8 | refine | |
| 69 | u8 | grade | |

### 7.2 ZC_DELETE_ITEM_FROM_BODY — `0x07FA` — 8 bytes

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | i16 | delete_type |
| 4 | u16 | index |
| 6 | i16 | count |

### 7.3 ZC_ITEM_THROW_ACK — `0x00AF` — 6 bytes

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | u16 | index |
| 4 | u16 | count (restante) |

### 7.4 ZC_ADD_ITEM_TO_STORE — `0x0B44` (MAIN >= 20200916)

Formato similar a ItemPickupAck sin campos result/favorite.

### 7.5 ZC_ADD_ITEM_TO_CART — `0x0B45` (MAIN >= 20200916)

Formato similar a ItemPickupAck sin campos result/favorite.

---

## 8. Equipamiento

### 8.1 ZC_REQ_WEAR_EQUIP_ACK — `0x0999` — 11 bytes

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | u16 | index |
| 4 | **u32** | wear_location |
| 8 | u16 | sprite_number |
| 10 | u8 | result |

### 8.2 ZC_REQ_TAKEOFF_EQUIP_ACK — `0x099A` — 9 bytes

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | u16 | index |
| 4 | **u32** | wear_location |
| 8 | u8 | flag |

---

## 9. Skills

> **NOTA**: Con PACKETVER_RE_NUM **no definido**, no hay campo `level2` en SkillData.

### 9.1 ZC_SKILLINFO_LIST — `0x010F` — Variable

Header:

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | u16 | packet_length |

Cada skill (15 bytes):

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | id |
| 2 | i32 | inf |
| 6 | u16 | level |
| 8 | u16 | sp |
| 10 | u16 | range |
| 12 | u8 | upgradable |

`skill_count = (packet_length - 4) / 15`

### 9.2 ZC_NOTIFY_SKILL — `0x0114` — 31 bytes

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | u16 | skill_id |
| 4 | u32 | src_id |
| 8 | u32 | target_id |
| 12 | u32 | start_time |
| 16 | i32 | src_speed |
| 20 | i32 | target_speed |
| 24 | i32 | damage |
| 28 | i16 | level |
| 30 | i16 | div (hits) |
| 32 | i8 | action |

### 9.3 ZC_ACK_TOUSESKILL — `0x0110`

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | i32 | btype |
| 6 | **u32** | item_id |
| 10 | u8 | flag |
| 11 | u8 | cause |

---

## 10. Items en el Suelo

### 10.1 ZC_ITEM_FALL_ENTRY — `0x0ADD` — 27 bytes

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | u32 | item_aid |
| 6 | **u32** | item_id |
| 10 | u16 | type_ |
| 12 | u8 | identified |
| 13 | i16 | x |
| 15 | i16 | y |
| 17 | u8 | sub_x |
| 18 | u8 | sub_y |
| 19 | i16 | amount |
| 21 | i8 | show_drop_effect |
| 22 | i16 | drop_effect_mode |

### 10.2 ZC_ITEM_DISAPPEAR — `0x00A1` — 6 bytes

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | u32 | item_aid |

---

## 11. NPC

### 11.1 ZC_SAY_DIALOG — `0x00B4` — Variable

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | u16 | packet_length |
| 4 | u32 | npc_id |
| 8 | char[] | message (len = packet_length - 8) |

### 11.2 ZC_WAIT_DIALOG — `0x00B5` — 6 bytes

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | u32 | npc_id |

### 11.3 ZC_CLOSE_DIALOG — `0x00B6` — 6 bytes

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | u32 | npc_id |

### 11.4 ZC_MENU_LIST — `0x00B7` — Variable

| Offset | Tipo | Campo | Nota |
|--------|------|-------|------|
| 0 | u16 | header | |
| 2 | u16 | packet_length | |
| 4 | u32 | npc_id | |
| 8 | char[] | menu | opciones separadas por `:`, null-terminated |

### 11.5 ZC_OPEN_EDITDLG — `0x0142` — 6 bytes (input numérico)

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | u32 | npc_id |

### 11.6 ZC_OPEN_EDITDLGSTR — `0x01D4` — 6 bytes (input texto)

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | u32 | npc_id |

### 11.7 ZC_PC_PURCHASE_ITEMLIST — `0x00C6` — Variable

Header: `u16 header + u16 packet_length`

Cada item (13 bytes):

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u32 | price |
| 4 | u32 | discount_price |
| 8 | u8 | item_type |
| 9 | **u32** | item_id |

### 11.8 ZC_PC_SELL_ITEMLIST — `0x00C7` — Variable

Header: `u16 header + u16 packet_length`

Cada item (10 bytes):

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | index |
| 2 | u32 | price |
| 6 | u32 | overcharge |

---

## 12. Party

### 12.1 ZC_PARTY_JOIN_REQ — `0x02C6` — 30 bytes

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | i32 | party_id |
| 6 | u8[24] | party_name |

### 12.2 ZC_PARTY_JOIN_REQ_ACK — `0x02C5` — 30 bytes

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | u8[24] | char_name |
| 26 | i32 | result |

### 12.3 ZC_GROUP_LIST — Variable

Header:

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | u16 | packet_length |
| 4 | u8[24] | party_name |

Cada miembro (54 bytes):

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u32 | aid |
| 4 | u32 | gid |
| 8 | u8[24] | name |
| 32 | u8[16] | map_name |
| 48 | u8 | leader |
| 49 | u8 | offline |
| 50 | i16 | class_ |
| 52 | i16 | base_level |

### 12.4 ZC_NOTIFY_HP_TO_GROUPM — `0x080E` — 14 bytes

| Offset | Tipo | Campo | Nota |
|--------|------|-------|------|
| 0 | u16 | header | |
| 2 | u32 | aid | |
| 6 | **i32** | hp | **NO i16** |
| 10 | **i32** | max_hp | **NO i16** |

### 12.5 ZC_NOTIFY_POSITION_TO_GROUPM — `0x0107` — 12 bytes

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | i32 | aid |
| 6 | i16 | x |
| 8 | i16 | y |

### 12.6 ZC_DELETE_MEMBER_FROM_GROUP — `0x0105` — 31 bytes

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | i32 | aid |
| 6 | u8[24] | name |
| 30 | i8 | result |

### 12.7 ZC_NOTIFY_CHAT_PARTY — `0x0109` — Sin cambios

### 12.8 ZC_NOTIFY_MEMBERINFO_TO_GROUPM — `0x0ABD` — 12 bytes

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | i32 | aid |
| 6 | i16 | job |
| 8 | i16 | level |

---

## 13. Chat

| Packet | ID | Tamaño | Cambio vs versiones anteriores |
|--------|----|--------|-------------------------------|
| ZC_WHISPER | **`0x09DE`** | 33+msg | Usar `0x09DE` (no `0x0097`) |
| ZC_ACK_WHISPER | **`0x09DF`** | 7b | Usar `0x09DF` (no `0x0098`) |
| ZC_NOTIFY_CHAT_PARTY | `0x0109` | var | Sin cambios |
| ZC_GUILD_CHAT | `0x017F` | var | Sin cambios |
| ZC_EMOTION | `0x00C0` | 7b | Sin cambios |

---

## 14. Trade

| Packet | ID | Tamaño | Cambio |
|--------|----|--------|--------|
| ZC_REQ_EXCHANGE_ITEM | `0x01F4` | 32b | `name[24]` al inicio |
| ZC_ACK_EXCHANGE_ITEM | **`0x01F5`** | 9b | Antes era `0x00E7` |
| ZC_ADD_EXCHANGE_ITEM | **`0x0B42`** | 62b | Antes era `0x00E9` |
| ZC_CONCLUDE_EXCHANGE | `0x00EC` | 3b | Sin cambios |
| ZC_CANCEL_EXCHANGE | `0x00EE` | 2b | Sin cambios |
| ZC_EXEC_EXCHANGE | `0x00F0` | 3b | Sin cambios |

---

## 15. Pet

| Packet | ID | Tamaño | Cambio |
|--------|----|--------|--------|
| ZC_PROPERTY_PET | `0x01A2` | 37b | +2 bytes campo `job` |
| ZC_FEED_PET | **`0x01A3`** | 7b | Antes era `0x01A4` |
| ZC_CHANGESTATE_PET | `0x01A4` | 11b | noop o handler |
| ZC_PETEGG_LIST | `0x01A6` | var | Sin cambios |
| ZC_PET_CATCH_RESULT | `0x01A0` | 3b | Sin cambios |
| ZC_PET_ACT | `0x01AA` | 10b | Sin cambios |

---

## 16. Homunculus

| Packet | ID | Tamaño | Cambio |
|--------|----|--------|--------|
| ZC_PROPERTY_HOMUN | **`0x0BA4`** | 85b | Antes era `0x022E`, formato nuevo |
| ZC_FEED_MER | `0x022F` | 7b | Verificar `u32` para food_item_id |
| ZC_HOMUN_ALIVE | — | — | **No existe en rAthena** — eliminar |

---

## 17. Misc

### 17.1 ZC_NOTIFY_CARTITEM_COUNTINFO — `0x0121` — 14 bytes

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | i16 | cur_count |
| 4 | i16 | max_count |
| 6 | i32 | cur_weight |
| 10 | i32 | max_weight |

### 17.2 ZC_NOTIFY_MAPINFO — `0x0189` — 4 bytes

| Offset | Tipo | Campo |
|--------|------|-------|
| 0 | u16 | header |
| 2 | i16 | type_ |

---

## Prioridad de Implementación Sugerida

1. **Unit Spawn** (`0x09FE`, `0x09FD`, `0x09FF`) — sin esto no se ven entidades
2. **Conexión** (`0x0A18`, `0x0080`, `0x0091`) — entrada al mapa
3. **Movimiento** (`0x0087`, `0x0088`) — caminar
4. **Stats** (`0x00BD`, `0x00B0`, `0x00B1`) — ver HP/SP/stats
5. **Combate** (`0x08C8`) — ver daño
6. **Inventario** (`0x0B41`, `0x07FA`) — items
7. **NPC** (`0x00B4`..`0x00B7`) — diálogos
8. **Chat** (`0x09DE`, `0x09DF`) — whisper
9. **Trade** — comercio
10. **Party** — grupo
11. **Skills** — habilidades
12. **Status Effects** — buffs/debuffs
13. **Pet / Homunculus** — mascotas
14. **Misc** — cart, map info

---

## Notas Importantes

- Todos los valores son **little-endian** (x86)
- Los tamaños indicados **incluyen** el header de 2 bytes salvo donde se indique lo contrario
- Packets de tamaño variable siempre tienen `packet_length` como segundo campo (incluye header + length)
- Validar con Wireshark/packet logger los byte layouts exactos antes de considerar la implementación completa
