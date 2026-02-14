# AnimaRO Packet Protocol Guide for Custom Rust Client

**PACKETVER: 20220406 | Type: PACKETVER_MAIN_NUM | Obfuscation: Keys=0x0 (inactive)**

Server source files:
- `src/custom/defines_pre.hpp` - PACKETVER definition
- `src/config/packets.hpp` - Packet config flags
- `src/map/core/clif_packetdb.hpp` - Base packet definitions (~2049 lines)
- `src/map/core/clif_shuffle.hpp` - Packet ID remapping per version
- `src/map/core/clif_obfuscation.hpp` - Encryption keys per version
- `src/map/core/clif.cpp` - Packet parsing and handling (~25800 lines)
- `src/map/core/packets.hpp` - Packet header ID constants and structures
- `src/map/core/packets_struct.hpp` - Binary packet structures
- `src/map/animaro/animaro_clif.hpp` - Custom AnimaRO protocol

---

## 1. Problema diagnosticado: packet 0x626F

```
[Warning]: clif_parse: Received unsupported packet (packet 0x626f, 16 bytes received), disconnecting session #8.
```

**Causa:** `0x626F` = 25199, el rango valido de paquetes es `0x0064` a `0x0FFF` (100 a 4095).
**Ubicacion del check:** `src/map/core/clif.cpp:25699`

```cpp
if (cmd > MAX_PACKET_DB || cmd < MIN_PACKET_DB || packet_db[cmd].len == 0) {
    ShowWarning("clif_parse: Received unsupported packet ...");
    // DESCONECTA LA SESION
}
```

Esto significa que tu cliente Rust esta enviando datos que no son un packet ID valido de RO.
Las causas posibles son:
1. **PACKETVER incorrecto** - korangar compila con un PACKETVER distinto a 20220406
2. **Obfuscation mismatch** - Tu cliente aplica XOR con keys != 0, pero el server usa keys = 0
3. **Packet shuffle incorrecto** - Tu cliente usa IDs de una version anterior donde estaban shuffled
4. **Protocolo roto** - El cliente envia datos raw que no son un packet RO (HTTP, basura, etc.)

**Para debuggear**, descomenta en `src/config/packets.hpp`:
```cpp
#define DUMP_INVALID_PACKET
#define DUMP_UNKNOWN_PACKET
```
Recompila y veras el hex dump completo de lo que envia tu cliente.

---

## 2. Obfuscation (Packet Encryption)

**Estado para PACKETVER 20220406:** ACTIVA en el server pero con keys = 0 (sin efecto).

```
Source: src/map/core/clif_obfuscation.hpp:421-422
Keys:   0x00000000, 0x00000000, 0x00000000
```

El server siempre aplica XOR al primer packet recibido (conexion):
```cpp
// src/map/core/clif.cpp:25694
cmd = (cmd ^ ((((clif_cryptKey[0] * clif_cryptKey[1]) + clif_cryptKey[2]) >> 16) & 0x7FFF));
// Con keys=0: cmd = cmd ^ ((((0*0)+0) >> 16) & 0x7FFF) = cmd ^ 0 = cmd (sin cambio)
```

Para sesiones establecidas, usa la key rotada del session data:
```cpp
// src/map/core/clif.cpp:25688
cmd = (cmd ^ ((sd->cryptKey >> 16) & 0x7FFF));
```

**Recomendacion:** Desactivar obfuscation en el server anadiendo a `src/custom/defines_pre.hpp`:
```cpp
#undef PACKET_OBFUSCATION
#undef PACKET_OBFUSCATION_WARN
```

O implementar en tu cliente Rust:
```rust
fn decrypt_packet_id(raw_cmd: u16, crypt_key: u32) -> u16 {
    raw_cmd ^ (((crypt_key >> 16) & 0x7FFF) as u16)
}
// Para conexion inicial: crypt_key = (key0 * key1) + key2 = 0
// Para sesion: crypt_key = session.crypt_key (rotado tras cada packet)
```

---

## 3. Secuencia de Conexion al Map Server

```
Login Server (6900) -> Char Server (6121) -> Map Server (5121)

=== CONEXION AL MAP SERVER ===

1. Client -> Server: 0x0436 WantToConnection (23 bytes)
   Estructura:
   [0-1]   packet_id    = 0x0436 (uint16 LE)
   [2-5]   account_id   = uint32 LE
   [6-9]   char_id      = uint32 LE
   [10-13] login_id1    = uint32 LE
   [14-17] client_tick   = uint32 LE
   [18-21] (padding)     = 4 bytes
   [22]    sex           = uint8 (0=female, 1=male)

2. Server -> Client: 0x0073/0x02EB/0x0A18 ZC_ACCEPT_ENTER
   Contiene: start_time, posicion XY, direccion, font

3. Client -> Server: 0x007D LoadEndAck (2 bytes)
   [0-1]   packet_id = 0x007D
   "He cargado el mapa, estoy listo"

4. Server -> Client: avalancha de datos iniciales
   - ZC_STATUS (0xBD) - Stats del personaje
   - ZC_NOTIFY_MOVEENTRY/NEWENTRY/STANDENTRY - Entidades en el mapa
   - Inventory, equipment, skills, etc.

5. Client -> Server: 0x0360 TickSend (6 bytes) - cada ~10 segundos
   [0-1]   packet_id   = 0x0360
   [2-5]   client_tick  = uint32 LE
```

---

## 4. Tabla Completa de Packets Client->Server (CZ)

Estos son los packet IDs FINALES que el server espera para PACKETVER 20220406.
La tabla de shuffle `clif_shuffle.hpp` override los IDs base de `clif_packetdb.hpp`.

Nota: `len = -1` significa longitud variable (los primeros 2 bytes tras el header son el length total).

### 4.1 Packets de Conexion y Sesion

| Packet ID | Len | Handler | Offsets | Descripcion |
|-----------|-----|---------|---------|-------------|
| `0x0436` | 23 | WantToConnection | 2,6,10,14,22 | Conexion al map server |
| `0x007D` | 2 | LoadEndAck | - | Mapa cargado, listo |
| `0x0360` | 6 | TickSend | 2 | Keepalive tick |
| `0x018A` | 4 | QuitGame | 2 | Salir del juego |
| `0x00B2` | 3 | Restart | 2 | Volver a char select |
| `0x044A` | 6 | client_version | 2 | Version del cliente |

### 4.2 Movimiento y Accion

| Packet ID | Len | Handler | Offsets | Descripcion |
|-----------|-----|---------|---------|-------------|
| `0x035F` | 5 | WalkToXY | 2 | Caminar a coordenada |
| `0x0361` | 5 | ChangeDir | 2,4 | Cambiar direccion |
| `0x0437` | 7 | ActionRequest | 2,6 | Atacar/sentarse/stand |
| `0x0118` | 2 | StopAttack | - | Parar ataque |

**ActionRequest action types (offset 6, uint8):**
- 0 = attack (once)
- 1 = pick up item
- 2 = sit down
- 3 = stand up
- 7 = attack (continuous)
- 12 = attack (?)

**WalkToXY format (3 bytes de coordenada empaquetada):**
```
byte[0] = x >> 2
byte[1] = (x << 6) | ((y >> 4) & 0x3F)
byte[2] = (y << 4) | (dir & 0x0F)
```

### 4.3 Chat y Comunicacion

| Packet ID | Len | Handler | Offsets | Descripcion |
|-----------|-----|---------|---------|-------------|
| `0x008C` | -1 | GlobalMessage | 2,4 | Chat publico |
| `0x0096` | -1 | WisMessage | 2,4,28 | Whisper/PM |
| `0x0108` | -1 | PartyMessage | 2,4 | Chat de party |
| `0x017E` | -1 | GuildMessage | 2,4 | Chat de guild |
| `0x02DB` | -1 | BattleChat | 2,4 | Chat de batalla |
| `0x098D` | -1 | clan_chat | 2,4 | Chat de clan |

**Formato de GlobalMessage (longitud variable):**
```
[0-1]   packet_id  = 0x008C
[2-3]   length     = uint16 LE (total packet length)
[4-..] message    = "CharName : mensaje\0" (null terminated)
```

### 4.4 NPC Interaction

| Packet ID | Len | Handler | Offsets | Descripcion |
|-----------|-----|---------|---------|-------------|
| `0x0090`* | var | NpcClicked | 0 | Click en NPC |
| `0x00B8` | 7 | NpcSelectMenu | 2,6 | Elegir opcion menu NPC |
| `0x00B9` | 6 | NpcNextClicked | 2 | NPC "next" |
| `0x0146`* | var | NpcCloseClicked | 0 | Cerrar dialogo NPC |
| `0x0143`* | var | NpcAmountInput | 0 | Input numerico NPC |
| `0x01D5`* | -1 | NpcStringInput | 0 | Input texto NPC |
| `0x00C8` | -1 | NpcBuyListSend | 2,4 | Comprar items a NPC |
| `0x00C9`* | -1 | NpcSellListSend | 2,4 | Vender items a NPC |
| `0x0447` | 2 | blocking_playcancel | - | Cancelar bloqueo NPC |

*Los packet IDs marcados con `*` usan HEADER_ constants resueltos en packets.hpp. Los IDs especificos dependen del PACKETVER. Consultar la seccion 8.

### 4.5 Items

| Packet ID | Len | Handler | Offsets | Descripcion |
|-----------|-----|---------|---------|-------------|
| `0x0362` | 6 | TakeItem | 2 | Recoger item del suelo |
| `0x0363` | 6 | DropItem | 2,4 | Tirar item |
| `0x0439` | 8 | UseItem | 2,4 | Usar item |
| `0x00A9`* | var | EquipItem | 0 | Equipar item |
| `0x00AB` | 4 | UnequipItem | 2 | Desequipar item |
| `0x0A35` | 4 | OneClickItemIdentify | 2 | Identificar item |

### 4.6 Skills

| Packet ID | Len | Handler | Offsets | Descripcion |
|-----------|-----|---------|---------|-------------|
| `0x0438` | 10 | UseSkillToId | 2,4,6 | Skill a un target |
| `0x0366` | 10 | UseSkillToPos | 2,4,6,8 | Skill a posicion XY |
| `0x0367` | 31 | UseSkillToPosMoreInfo | 2,4,6,8,10 | Skill pos + info extra |
| `0x0112` | 4 | SkillUp | 2 | Subir nivel de skill |
| `0x011B`* | var | UseSkillMap | 0 | Skill de teleport/warp |
| `0x0AF4` | 11 | UseSkillToPos(v2) | 2,4,6,8,10 | Skill pos nueva version |

**UseSkillToId structure (10 bytes):**
```
[0-1]  packet_id  = 0x0438
[2-3]  skill_lv   = uint16 LE
[4-5]  skill_id   = uint16 LE
[6-9]  target_id  = uint32 LE
```

**UseSkillToPos structure (10 bytes):**
```
[0-1]  packet_id  = 0x0366
[2-3]  skill_lv   = uint16 LE
[4-5]  skill_id   = uint16 LE
[6-7]  x          = uint16 LE
[8-9]  y          = uint16 LE
```

### 4.7 Stats y Status

| Packet ID | Len | Handler | Offsets | Descripcion |
|-----------|-----|---------|---------|-------------|
| `0x00BB` | 5 | StatusUp | 2,4 | Subir stat (str/agi/vit/int/dex/luk) |
| `0x0B24`* | var | TraitStatusUp | 0 | Subir trait stat (4th class) |

**StatusUp structure:**
```
[0-1]  packet_id = 0x00BB
[2-3]  status_id = uint16 LE (13=str, 14=agi, 15=vit, 16=int, 17=dex, 18=luk)
[4]    amount    = uint8
```

### 4.8 Trade

| Packet ID | Len | Handler | Offsets | Descripcion |
|-----------|-----|---------|---------|-------------|
| `0x00E4` | 6 | TradeRequest | 2 | Solicitar trade |
| `0x00E6` | 3 | TradeAck | 2 | Aceptar/rechazar trade |
| `0x00E8`* | var | TradeAddItem | 0 | Agregar item al trade |
| `0x00EB` | 2 | TradeOk | - | Confirmar trade |
| `0x00ED` | 2 | TradeCancel | - | Cancelar trade |
| `0x00EF` | 2 | TradeCommit | - | Ejecutar trade |

### 4.9 Storage (Kafra)

| Packet ID | Len | Handler | Offsets | Descripcion |
|-----------|-----|---------|---------|-------------|
| `0x0364` | 8 | MoveToKafra | 2,4 | Item a storage |
| `0x0365` | 8 | MoveFromKafra | 2,4 | Item de storage |
| `0x00F7` | 2 | CloseKafra | - | Cerrar storage |

**MoveToKafra structure:**
```
[0-1]  packet_id = 0x0364
[2-3]  index     = uint16 LE (inventory index)
[4-7]  amount    = uint32 LE
```

### 4.10 Party

| Packet ID | Len | Handler | Offsets | Descripcion |
|-----------|-----|---------|---------|-------------|
| `0x00F9`* | var | CreateParty | 0 | Crear party |
| `0x01E8`* | var | CreateParty2 | 0 | Crear party v2 |
| `0x02C4` | 26 | PartyInvite2 | 2 | Invitar a party |
| `0x02C7`* | var | ReplyPartyInvite2 | 0 | Responder invitacion |
| `0x0100`* | var | LeaveParty | 0 | Salir de party |
| `0x0103`* | var | RemovePartyMember | 0 | Expulsar de party |
| `0x0102` | 6 | PartyChangeOption | 2 | Cambiar opciones party |
| `0x07DA` | 6 | PartyChangeLeader | 2 | Cambiar lider |

### 4.11 Guild

| Packet ID | Len | Handler | Offsets | Descripcion |
|-----------|-----|---------|---------|-------------|
| `0x0165` | 30 | CreateGuild | 2,6 | Crear guild |
| `0x0168` | var | GuildInvite | 0 | Invitar a guild |
| `0x016B`* | var | GuildReplyInvite | 0 | Responder invitacion |
| `0x0159`* | var | GuildLeave | 0 | Salir de guild |
| `0x015B`* | var | GuildExpulsion | 0 | Expulsar |
| `0x015D`* | var | GuildBreak | 0 | Disolver guild |
| `0x014D` | 2 | GuildCheckMaster | - | Verificar master |
| `0x014F` | 6 | GuildRequestInfo | 2 | Pedir info guild |
| `0x0151` | 6 | GuildRequestEmblem | 2 | Pedir emblema |
| `0x016E` | 186 | GuildChangeNotice | 2,6,66 | Cambiar aviso |
| `0x017E` | -1 | GuildMessage | 2,4 | Chat guild |

### 4.12 Vending

| Packet ID | Len | Handler | Offsets | Descripcion |
|-----------|-----|---------|---------|-------------|
| `0x01B2` | -1 | OpenVending | 2,4,84,85 | Abrir tienda |
| `0x012E` | 2 | CloseVending | - | Cerrar tienda |
| `0x0130` | 6 | VendingListReq | 2 | Ver tienda de otro |
| `0x0134`* | -1 | PurchaseReq | 0 | Comprar en tienda |

### 4.13 Buying Store

| Packet ID | Len | Handler | Offsets | Descripcion |
|-----------|-----|---------|---------|-------------|
| `0x0811` | -1 | ReqOpenBuyingStore | 2,4,8,9,89 | Abrir tienda compra |
| `0x0815` | 2 | ReqCloseBuyingStore | - | Cerrar tienda compra |
| `0x0817` | 6 | ReqClickBuyingStore | 2 | Click tienda compra |
| `0x0819` | -1 | ReqTradeBuyingStore | 2,4,8,12 | Vender en tienda |

### 4.14 Search Store

| Packet ID | Len | Handler | Offsets | Descripcion |
|-----------|-----|---------|---------|-------------|
| `0x0835` | -1 | SearchStoreInfo | 2,4,5,9,13,14,15 | Buscar tiendas |
| `0x0838` | 2 | SearchStoreInfoNextPage | - | Siguiente pagina |
| `0x083C` | var | SearchStoreInfoListItemClick | 0 | Click resultado |

### 4.15 Friends

| Packet ID | Len | Handler | Offsets | Descripcion |
|-----------|-----|---------|---------|-------------|
| `0x0202` | 26 | FriendsListAdd | 2 | Agregar amigo |
| `0x0203` | 10 | FriendsListRemove | 2,6 | Eliminar amigo |
| `0x0208` | 14 | FriendsListReply | 2,6,10 | Responder solicitud |

### 4.16 Homunculus

| Packet ID | Len | Handler | Offsets | Descripcion |
|-----------|-----|---------|---------|-------------|
| `0x022D` | 5 | HomMenu | 2,4 | Menu homunculus |
| `0x0232`* | var | HomMoveTo | 0 | Mover homunculus |
| `0x0233` | 11 | HomAttack | 2,6,10 | Homunculus ataca |
| `0x0234` | 6 | HomMoveToMaster | 2 | Hom vuelve al master |

### 4.17 Pet

| Packet ID | Len | Handler | Offsets | Descripcion |
|-----------|-----|---------|---------|-------------|
| `0x019F` | 6 | CatchPet | 2 | Capturar pet |
| `0x01A1` | 3 | PetMenu | 2 | Menu pet |
| `0x01A5` | 26 | ChangePetName | 2 | Renombrar pet |
| `0x01A7` | 4 | SelectEgg | 2 | Seleccionar huevo |
| `0x01A9` | 6 | SendEmotion | 2 | Emocion del pet |

### 4.18 Cart

| Packet ID | Len | Handler | Offsets | Descripcion |
|-----------|-----|---------|---------|-------------|
| `0x0126`* | var | PutItemToCart | 0 | Item a cart |
| `0x0127`* | var | GetItemFromCart | 0 | Item de cart |
| `0x01AF` | 4 | ChangeCart | 2 | Cambiar tipo cart |

### 4.19 Chat Room

| Packet ID | Len | Handler | Offsets | Descripcion |
|-----------|-----|---------|---------|-------------|
| `0x00D5`* | -1 | CreateChatRoom | 0 | Crear sala |
| `0x00D9`* | var | ChatAddMember | 0 | Entrar a sala |
| `0x00DE`* | -1 | ChatRoomStatusChange | 0 | Cambiar config |
| `0x00E0` | 30 | ChangeChatOwner | 2,6 | Cambiar dueno |
| `0x00E2` | 26 | KickFromChat | 2 | Expulsar |
| `0x00E3` | 2 | ChatLeave | - | Salir de sala |

### 4.20 Emotion

| Packet ID | Len | Handler | Offsets | Descripcion |
|-----------|-----|---------|---------|-------------|
| `0x00BF`* | var | Emotion | 0 | Enviar emoticon |

### 4.21 Mail (RODEX)

| Packet ID | Len | Handler | Offsets | Descripcion |
|-----------|-----|---------|---------|-------------|
| `0x09E8` | 11 | Mail_refreshinbox | 2,3 | Abrir buzon |
| `0x09E9` | 2 | (dull) | - | Cerrar buzon |
| `0x09EA` | 11 | Mail_read | 2,3 | Leer correo |
| `0x0A6E` | -1 | Mail_send | 2,4,28,52,60,62,64,68 | Enviar correo (v2) |
| `0x09F1` | 11 | Mail_getattach (zeny) | 0 | Recoger zeny |
| `0x09F3` | 11 | Mail_getattach (item) | 0 | Recoger items |
| `0x09F5` | 11 | Mail_delete | 0 | Eliminar correo |
| `0x0A03` | 2 | Mail_cancelwrite | - | Cancelar escritura |
| `0x0A04` | 6 | Mail_setattach | 2,4 | Adjuntar item |
| `0x0A06` | 6 | Mail_winopen | 2,4 | Quitar adjunto |
| `0x0A08` | 26 | Mail_beginwrite | 0 | Comenzar escritura |
| `0x0A13`* | var | Mail_Receiver_Check | 0 | Verificar destinatario |
| `0x0AC0` | 26 | Mail_refreshinbox | 2,10 | Refrescar (v2) |
| `0x0AC1` | 26 | Mail_refreshinbox | 2,10 | Siguiente pagina |

### 4.22 Quest

| Packet ID | Len | Handler | Offsets | Descripcion |
|-----------|-----|---------|---------|-------------|
| `0x02B6`* | var | questStateAck | 0 | Activar/desactivar quest |

### 4.23 Bank

| Packet ID | Len | Handler | Offsets | Descripcion |
|-----------|-----|---------|---------|-------------|
| `0x09A7`* | var | BankDeposit | 0 | Depositar zeny |
| `0x09A9`* | var | BankWithdraw | 0 | Retirar zeny |
| `0x09A5`* | var | BankCheck | 0 | Consultar saldo |
| `0x09B6`* | var | BankOpen | 0 | Abrir banco |
| `0x09B8`* | var | BankClose | 0 | Cerrar banco |

### 4.24 Booking System

| Packet ID | Len | Handler | Offsets | Descripcion |
|-----------|-----|---------|---------|-------------|
| `0x0802` | 18 | PartyBookingRegisterReq | 2,4 | Registrar booking |
| `0x08E7` | 10 | PartyBookingSearchReq | 2,4,6,8,12 | Buscar booking |
| `0x08E9` | 2 | PartyBookingDeleteReq | - | Eliminar booking |
| `0x08EB` | 39 | PartyBookingUpdateReq | 2 | Actualizar booking |

### 4.25 Cash Shop

| Packet ID | Len | Handler | Offsets | Descripcion |
|-----------|-----|---------|---------|-------------|
| `0x0844` | 2 | cashshop_open_request | - | Abrir cash shop |
| `0x0848`* | -1 | cashshop_buy | 0 | Comprar en cash shop |
| `0x084A` | 2 | cashshop_close | - | Cerrar cash shop |
| `0x08C9` | 2 | cashshop_list_request | - | Pedir lista cash shop |
| `0x0288` | -1 | npccashshop_buy | 2,4,8,10 | Comprar NPC cash |

### 4.26 Equipswitch

| Packet ID | Len | Handler | Offsets | Descripcion |
|-----------|-----|---------|---------|-------------|
| `0x0A97` | 8 | equipswitch_add | 2,4 | Agregar equipswitch |
| `0x0A99` | 4 | equipswitch_remove | 2,4 | Quitar equipswitch |
| `0x0A9C` | 2 | equipswitch_request | - | Ejecutar switch |
| `0x0ACE` | 4 | equipswitch_request_single | 0 | Switch individual |

### 4.27 Achievement / Title

| Packet ID | Len | Handler | Offsets | Descripcion |
|-----------|-----|---------|---------|-------------|
| `0x0A25` | 6 | AchievementCheckReward | 0 | Reclamar logro |
| `0x0A2E` | 6 | change_title | 0 | Cambiar titulo |

### 4.28 Attendance / UI

| Packet ID | Len | Handler | Offsets | Descripcion |
|-----------|-----|---------|---------|-------------|
| `0x0A68` | 3 | open_ui | 2 | Abrir UI generica |
| `0x0AEF` | 2 | attendance_request | - | Solicitar asistencia |
| `0x0AE8` | 2 | changedress | - | Cambiar disfraz |

### 4.29 Hotkeys

| Packet ID | Len | Handler | Offsets | Descripcion |
|-----------|-----|---------|---------|-------------|
| `0x0B21` | var | Hotkey (v2) | 0 | Cambiar hotkey |
| `0x0B22` | var | HotkeyRowShift (v2) | 0 | Rotar barra hotkey |

### 4.30 Misc

| Packet ID | Len | Handler | Offsets | Descripcion |
|-----------|-----|---------|---------|-------------|
| `0x0368` | 6 | GetCharNameRequest | 2 | Pedir nombre entidad |
| `0x0369` | 6 | SolveCharName | 2 | Resolver char name |
| `0x00C1` | 2 | HowManyConnections | - | Usuarios online |
| `0x0193` | 6 | CheckEquipment | 2 | Verificar equipo |
| `0x00CF` | 27 | PMIgnore | 2,26 | Ignorar whisper |
| `0x00D3` | 2 | PMIgnoreList | - | Lista ignorados |
| `0x0292` | 2 | AutoRevive | - | Auto-revivir (token) |
| `0x011D` | 2 | RequestMemo | - | Guardar warp point |
| `0x0907`* | var | MoveItem | 0 | Mover item tab inventario |
| `0x07E4` | -1 | ItemListWindowSelected | 2,4,8,12 | Seleccion items ventana |
| `0x02D8` | 10 | configuration | 2,6 | Configuracion (show equip etc) |
| `0x02D6` | 6 | ViewPlayerEquip | 2 | Ver equipo de otro |
| `0x0A77` | 15 | camerainfo | 0 | Info de camara |
| `0x023B` | 36 | StoragePassword | 0 | Password de storage |

### 4.31 GM Commands

| Packet ID | Len | Handler | Offsets | Descripcion |
|-----------|-----|---------|---------|-------------|
| `0x00CC` | 6 | GMKick | 2 | Kick jugador |
| `0x00CE` | 2 | GMKickAll | - | Kick todos |
| `0x0149` | 9 | GMReqNoChat | 2,6,7 | Mute jugador |
| `0x019D` | 6 | GMHide | 2 | Ocultar GM |
| `0x01BA` | 26 | GMShift | 2 | Teletransportar a jugador |
| `0x01BC` | 26 | GMRecall | 2 | Convocar jugador |
| `0x0198` | 8 | GMChangeMapType | 2,4,6 | Cambiar tipo mapa |
| `0x013F` | 26 | GM_Item_Monster | 2 | Crear item/mob (legacy) |
| `0x09CE` | 102 | GM_Item_Monster | 2 | Crear item/mob (nuevo) |
| `0x0842` | 6 | GMRecall2 | 2 | Recall v2 |
| `0x0843` | 6 | GMRemove2 | 2 | Remove v2 |
| `0x01DF` | 6 | GMReqAccountName | 2 | Pedir nombre de cuenta |
| `0x0212` | 26 | GMRc | 2 | GM recall command |
| `0x0213` | 26 | Check | 2 | Check command |

---

## 5. AnimaRO Custom Packets

```
Source: src/map/core/packets.hpp:31-32
Source: src/map/core/clif.cpp:25817-25824
Source: src/map/animaro/animaro_clif.hpp
Source: src/map/animaro/animaro_clif.cpp
```

### 5.1 CZ_ANIMA_ACTION (0x0F01) - Client -> Server

```
[0-1]  packet_id = 0x0F01
[2-3]  action_type = uint16 LE
Total: 4 bytes
```

Handler: `clif_parse_AnimaAction`

### 5.2 ZC_ANIMA_DATA (0x0F02) - Server -> Client

```
[0-1]  packet_id = 0x0F02
[2-3]  length    = uint16 LE (variable length)
[4-..] json_data = string (JSON payload)
```

Funciones del server que envian este packet:
- `animaro_clif_send_stats(sd)` - Stats de Anima
- `animaro_clif_send_anima_stats(sd)` - Stats de Anima detallados
- `animaro_clif_send_anima_skill(sd, skill_id, level)` - Skills de Anima
- `animaro_clif_send_ki(sd)` - Ki/energia
- `animaro_clif_send_packet(sd, prefix, data)` - Packet generico

### 5.3 Client Hook UI Packet (0x0187)

```
[0-1]  packet_id = 0x0187
[2-5]  data      = 4 bytes
Total: 6 bytes
```

Handler: `clif_parse_AnimaAction` (reutiliza el handler de AnimaAction)

---

## 6. Formato General de Packets

### Packet de longitud fija
```
[0-1] packet_id (uint16 LE)
[2-..] data (longitud definida en packet_db)
```

### Packet de longitud variable (len = -1)
```
[0-1] packet_id (uint16 LE)
[2-3] total_length (uint16 LE, incluye header)
[4-..] data
```

### Endianness
Todo es **Little Endian** (LE).

### Tipos de datos comunes
| Tipo | Tamano | Descripcion |
|------|--------|-------------|
| uint8 | 1 byte | Byte sin signo |
| uint16 | 2 bytes | Short sin signo, LE |
| uint32 | 4 bytes | Int sin signo, LE |
| int16 | 2 bytes | Short con signo, LE |
| int32 | 4 bytes | Int con signo, LE |
| char[N] | N bytes | String, null-padded |

---

## 7. Referencia Rapida: IDs Finales Post-Shuffle para PACKETVER > 20180307

Esta es la tabla definitiva del shuffle (`clif_shuffle.hpp:4723-4761`).
Estos IDs REEMPLAZAN los IDs base del packetdb para las funciones listadas:

```
0x0202 = FriendsListAdd (26 bytes)
0x022D = HomMenu (5 bytes)
0x023B = StoragePassword (36 bytes)
0x02C4 = PartyInvite2 (26 bytes)
0x035F = WalkToXY (5 bytes)
0x0360 = TickSend (6 bytes)
0x0361 = ChangeDir (5 bytes)
0x0362 = TakeItem (6 bytes)
0x0363 = DropItem (6 bytes)
0x0364 = MoveToKafra (8 bytes)
0x0365 = MoveFromKafra (8 bytes)
0x0366 = UseSkillToPos (10 bytes)
0x0367 = UseSkillToPosMoreInfo (31 bytes)  [PACKETVER_MAIN >= 20190904]
0x0368 = GetCharNameRequest (6 bytes)
0x0369 = SolveCharName (6 bytes)
0x0436 = WantToConnection (23 bytes)  [PACKETVER_MAIN >= 20220330]
0x0437 = ActionRequest (7 bytes)
0x0438 = UseSkillToId (10 bytes)
0x07E4 = ItemListWindowSelected (variable)
0x0802 = PartyBookingRegisterReq (18 bytes)
0x0811 = ReqOpenBuyingStore (variable)
0x0815 = ReqCloseBuyingStore (2 bytes)
0x0817 = ReqClickBuyingStore (6 bytes)
0x0819 = ReqTradeBuyingStore (variable)
0x0835 = SearchStoreInfo (variable)
0x0838 = SearchStoreInfoNextPage (2 bytes)
0x083C = SearchStoreInfoListItemClick (variable)
```

---

## 8. HEADER_ Constants resueltos (packets.hpp)

Algunos packets en `clif_packetdb.hpp` usan macros `HEADER_*` en vez de IDs numericos.
Estos son los valores para PACKETVER 20220406:

| Macro | Packet ID | Descripcion |
|-------|-----------|-------------|
| `HEADER_CZ_CONTACTNPC` | 0x0090 | Click NPC |
| `HEADER_CZ_BROADCAST` | 0x0099 | GM broadcast |
| `HEADER_CZ_REQ_WEAR_EQUIP` | 0x0998 | Equipar (PACKETVER >= ?) |
| `HEADER_CZ_ACK_SELECT_DEALTYPE` | 0x00C5 | NPC buy/sell select |
| `HEADER_CZ_PC_SELL_ITEMLIST` | 0x00C9 | Vender a NPC |
| `HEADER_CZ_SETTING_WHISPER_STATE` | varies | Ignorar todos PM |
| `HEADER_CZ_CREATE_CHATROOM` | 0x00D5 | Crear chat room |
| `HEADER_CZ_REQ_ENTER_ROOM` | varies | Entrar chat room |
| `HEADER_CZ_CHANGE_CHATROOM` | varies | Cambiar chat room |
| `HEADER_CZ_ADD_EXCHANGE_ITEM` | varies | Agregar item trade |
| `HEADER_CZ_MAKE_GROUP` | varies | Crear party |
| `HEADER_CZ_REQ_JOIN_GROUP` | 0x00FC | Invitar party |
| `HEADER_CZ_JOIN_GROUP` | 0x00FF | Responder party |
| `HEADER_CZ_REQ_LEAVE_GROUP` | 0x0100 | Salir party |
| `HEADER_CZ_REQ_EXPEL_GROUP_MEMBER` | 0x0103 | Expulsar party |
| `HEADER_CZ_SELECT_WARPPOINT` | varies | Seleccionar warp |
| `HEADER_CZ_MOVE_ITEM_FROM_BODY_TO_CART` | varies | Inv -> Cart |
| `HEADER_CZ_MOVE_ITEM_FROM_CART_TO_BODY` | varies | Cart -> Inv |
| `HEADER_CZ_REQ_CHANGE_MEMBERPOS` | 0x0155 | Cambiar pos guild |
| `HEADER_CZ_REQ_LEAVE_GUILD` | 0x0159 | Salir guild |
| `HEADER_CZ_REQ_BAN_GUILD` | 0x015B | Expulsar guild |
| `HEADER_CZ_REQ_DISORGANIZE_GUILD` | 0x015D | Disolver guild |
| `HEADER_CZ_REQ_JOIN_GUILD` | 0x0168 | Invitar guild |
| `HEADER_CZ_JOIN_GUILD` | 0x016B | Responder guild inv |
| `HEADER_CZ_REQ_EMOTION` | 0x00BF | Emoticon |
| `HEADER_CZ_PARTY_JOIN_REQ` | 0x02C4 | Invitar party v2 |
| `HEADER_CZ_PARTY_JOIN_REQ_ACK` | 0x02C7 | Responder party v2 |
| `HEADER_CZ_PARTY_CONFIG` | varies | Config party |
| `HEADER_CZ_INPUT_EDITDLG` | varies | NPC number input |
| `HEADER_CZ_INPUT_EDITDLGSTR` | varies | NPC string input |
| `HEADER_CZ_CLOSE_DIALOG` | varies | Cerrar dialogo NPC |
| `HEADER_CZ_ACTIVE_QUEST` | 0x02B6 | Quest active toggle |
| `HEADER_CZ_REQ_RANKING` | 0x097C | Rankings |
| `HEADER_CZ_REQ_MERGE_ITEM` | 0x096E | Merge items |
| `HEADER_CZ_INVENTORY_TAB` | 0x0907 | Mover item tab |
| `HEADER_CZ_SE_PC_BUY_CASHITEM_LIST` | 0x0848 | Cash shop buy |
| `HEADER_CZ_SSILIST_ITEM_CLICK` | varies | Store search click |
| `HEADER_CZ_REQ_STYLE_CHANGE2` | varies | Stylist (v2) |
| `HEADER_CZ_ANIMA_ACTION` | 0x0F01 | AnimaRO custom |
| `HEADER_ZC_ANIMA_DATA` | 0x0F02 | AnimaRO custom |

---

## 9. Checklist de Implementacion para Cliente Rust

### Prioridad 1 - Funcional basico (poder loguearte y moverte)
- [ ] `0x0436` WantToConnection (23 bytes, con padding de 4 bytes nuevo)
- [ ] `0x007D` LoadEndAck
- [ ] `0x0360` TickSend (cada ~10s)
- [ ] `0x035F` WalkToXY
- [ ] `0x0361` ChangeDir
- [ ] `0x0437` ActionRequest (atacar, sentarse)
- [ ] `0x018A` QuitGame
- [ ] Parsear todos los ZC packets que el server envia tras LoadEndAck

### Prioridad 2 - Interaccion basica
- [ ] `0x008C` GlobalMessage (chat)
- [ ] `0x0096` WisMessage (whisper)
- [ ] `0x0090` NpcClicked
- [ ] `0x00B8` NpcSelectMenu
- [ ] `0x00B9` NpcNextClicked
- [ ] `0x0146` NpcCloseClicked
- [ ] `0x0362` TakeItem
- [ ] `0x0363` DropItem
- [ ] `0x0439` UseItem
- [ ] `0x00A9`/`0x0998` EquipItem
- [ ] `0x00AB` UnequipItem
- [ ] `0x0368` GetCharNameRequest

### Prioridad 3 - Combate
- [ ] `0x0438` UseSkillToId
- [ ] `0x0366` UseSkillToPos
- [ ] `0x0367` UseSkillToPosMoreInfo
- [ ] `0x0112` SkillUp
- [ ] `0x00BB` StatusUp
- [ ] `0x0118` StopAttack

### Prioridad 4 - Economia
- [ ] `0x00E4-EF` Trade system completo
- [ ] `0x0364-0365` Storage (Kafra)
- [ ] `0x00F7` CloseKafra
- [ ] `0x01B2` OpenVending
- [ ] `0x012E` CloseVending
- [ ] `0x0130` VendingListReq
- [ ] NPC buy/sell

### Prioridad 5 - Social
- [ ] Party system (crear, invitar, salir)
- [ ] Guild system
- [ ] Friends system
- [ ] Chat rooms
- [ ] Mail (RODEX)

### Prioridad 6 - AnimaRO Custom
- [ ] `0x0F01` CZ_ANIMA_ACTION
- [ ] `0x0F02` ZC_ANIMA_DATA (parsear JSON)
- [ ] `0x0187` Client Hook UI

---

## 10. Troubleshooting

### "Received unsupported packet" con ID > 0x0FFF
Tu cliente envia datos que no son un packet RO valido. Verifica:
1. Que el primer byte enviado al map-server sea el packet 0x0436
2. Que no estes enviando HTTP o algun otro protocolo
3. Que la conexion TCP va directamente al puerto del map-server (5121)

### "Received unsupported packet" con ID valido pero desconocido
El server no tiene handler para ese packet ID. Posibles causas:
1. Tu cliente usa IDs de una version anterior (pre-2018, shuffled)
2. Tu cliente usa IDs de Hercules en vez de rAthena

### Session desconecta inmediatamente tras conectar
1. Verifica que envias `0x0436` con 23 bytes (no 19) - PACKETVER >= 20220330 anade 4 bytes de padding
2. Verifica account_id, char_id, login_id1 coinciden con los del login/char server
3. Verifica el campo sex (offset 22, no 18)

### "PACKET_OBFUSCATION enabled on server, disabled on client"
El server detecta que tu cliente no aplica XOR. Soluciones:
1. Desactivar obfuscation en el server (`#undef PACKET_OBFUSCATION`)
2. O implementar XOR en tu cliente (con keys 0x0 es transparente)

### Server envia packets que tu cliente no entiende
Todos los ZC packets estan definidos en `packets_struct.hpp`. Las estructuras binarias exactas dependen del PACKETVER y cambian frecuentemente. Consulta ese archivo para los structs exactos.
