# Informe para el Agente rAthena - Soporte de Packets para Korangar

## Contexto

El cliente Korangar (packet version `20220406`) tiene implementadas las siguientes features que requieren soporte del servidor:

0. **Login / Char / Map** (conexion, autenticacion, seleccion de personaje)
1. **Chat / Social** (whisper, party chat, guild chat, emotes)
2. **Trade** (intercambio completo entre jugadores)
3. **Pet** (alimentar, info, huevos, captura, emociones)
4. **Homunculus** (alimentar, info, estado alive/dead)
5. **Minimap** (no requiere cambios en servidor - es client-side)

El cliente ya tiene TODOS los packets definidos, enviados y procesados correctamente. Este informe detalla los formatos exactos extraidos del codigo fuente del cliente para que rAthena los soporte.

---

## 0. Login / Character / Map Server

### 0.1 Configuracion del Cliente (sclientinfo.xml)

El cliente carga la configuracion del servidor desde `data/sclientinfo.xml` (o `data/clientinfo.xml` como fallback). Formato XML:

```xml
<?xml version="1.0" encoding="euc-kr" ?>
<clientinfo>
    <desc>AnimaRO</desc>
    <servicetype>america</servicetype>
    <servertype>sakray</servertype>
    <extendedslot />
    <connection>
        <display>AnimaRO</display>
        <desc>AnimaRO Server</desc>
        <address>127.0.0.1</address>
        <port>6900</port>
        <version>55</version>
        <langtype>1</langtype>
        <registrationweb>http://animaro.com/register</registrationweb>
        <packet_version>20220406</packet_version>
    </connection>
</clientinfo>
```

**Campos criticos**:
- `<address>` y `<port>`: IP y puerto del login server
- `<version>`: DEBE coincidir con `client_version_to_connect` en `login_athena.conf`
- `<packet_version>`: Campo especifico de Korangar. DEBE ser `20220406`
- `<servicetype>`: `korea`, `america`, etc. Afecta comportamiento del cliente
- `<servertype>`: `primary`, `sakray`, `local`, `pk`

**IMPORTANTE**: `<passwordencrypt />` y `<passwordencrypt2 />` son opcionales. Si estan presentes, el cliente encripta las passwords. Son incompatibles con `use_MD5_passwords` en `login_athena.conf`.

### 0.2 Secuencia Completa de Conexion

```
CLIENTE                          LOGIN SERVER
  |                                   |
  |--- LoginServerLoginPacket ------->|  (0x0064)
  |    [version, username, password]  |
  |                                   |
  |<-- LoginServerLoginSuccessPacket -|  (0x0AC4)
  |    [account_id, login_id1,        |
  |     login_id2, sex, auth_token,   |
  |     character_server_list]        |
  |                                   |
  |  (keepalive cada 58s: 0x0200)     |
  |                                   |

CLIENTE                          CHAR SERVER
  |                                   |
  |--- CharacterServerLoginPacket --->|  (0x0065)
  |    [account_id, login_id1,        |
  |     login_id2, sex]               |
  |                                   |
  |<-- (raw 4 bytes account_id) ------|  (SIN header!)
  |<-- CharServerLoginSuccessPacket --|  (0x082D)
  |<-- CharacterBanListPacket --------|  (0x020D) noop
  |<-- CharacterSlotPagePacket -------|  (0x09A0) noop
  |<-- LoginPincodePacket ------------|  (0x08B9) noop
  |<-- Packet0b18 --------------------|  (0x0B18) noop
  |                                   |
  |--- RequestCharacterListPacket --->|  (0x09A1)
  |<-- CharacterListSuccessPacket ----|  (0x0B72)
  |    [character_information[]]      |
  |                                   |
  |--- SelectCharacterPacket -------->|  (0x0066)
  |<-- CharSelectionSuccessPacket ----|  (0x0AC5)
  |    [character_id, map_name,       |
  |     map_server_ip, map_server_port]|
  |                                   |
  |  (keepalive cada 10s: 0x0187)     |
  |                                   |

CLIENTE                          MAP SERVER
  |                                   |
  |--- MapServerLoginPacket --------->|  (0x0436)
  |    [account_id, character_id,     |
  |     login_id1, client_tick, sex]  |
  |                                   |
  |<-- MapServerLoginSuccessPacket ---|  (0x02EB)
  |    [client_tick, position, font]  |
  |                                   |
  |  (cliente carga el mapa)          |
  |                                   |
  |--- MapLoadedPacket -------------->|  (0x007D)
  |                                   |
  |<-- (inicializacion: inventario,   |
  |     stats, skills, friends, etc.) |
  |                                   |
  |  (keepalive/tick cada 10s)        |
```

### 0.3 Detalle de Packets - Login Server

**CA_LOGIN (0x0064)** - Cliente se autentica:
```
[2 bytes]  header = 0x0064
[4 bytes]  version (u32, default 0)
[24 bytes] username (string, null-padded a 24 bytes)
[24 bytes] password (string, null-padded a 24 bytes)
[1 byte]   client_type (u8, default 0)
```
Total: 55 bytes

**AC_ACCEPT_LOGIN (0x0AC4)** - Login exitoso (variable-length):
```
[2 bytes]  header = 0x0AC4
[2 bytes]  packet_len
[4 bytes]  login_id1 (u32)
[4 bytes]  account_id (u32)
[4 bytes]  login_id2 (u32)
[4 bytes]  ip_address (u32, deprecated, siempre 0)
[24 bytes] name (deprecated, siempre 0)
[2 bytes]  unknown (u16, siempre 0)
[1 byte]   sex (u8: 0=female, 1=male)
[17 bytes] auth_token
[remaining] character_server_information[] (ver abajo)
```

Cada `CharacterServerInformation` dentro del array:
```
[4 bytes]  server_ip (u32, IPv4 en network byte order)
[2 bytes]  server_port (u16)
[20 bytes] server_name (string, null-padded a 20 bytes)
[2 bytes]  user_count (u16)
[2 bytes]  server_type (u16)
[2 bytes]  display_new (u16)
[128 bytes] unknown (padding, siempre 0)
```
Cada entrada = 160 bytes. Cantidad de servers = (packet_len - 64) / 160

**AC_REFUSE_LOGIN (0x083E)** - Login fallido:
```
[2 bytes] header = 0x083E
[1 byte]  reason (u8):
          0 = unregistered id
          1 = incorrect password
          2 = id expired
          3 = rejected from server
          4 = blocked by gm team
          5 = game outdated
          6 = login prohibited until
          7 = server full
          8 = company account limit reached
```

**SC_NOTIFY_BAN (0x0081)** - Desconexion/ban:
```
[2 bytes] header = 0x0081
[1 byte]  reason (u8):
          1 = server closed
          2 = already logged in
          8 = already online
```

**Keepalive (0x0200)** - Cada 58 segundos:
```
[2 bytes]  header = 0x0200
[24 bytes] user_id (siempre 0)
```

### 0.4 Detalle de Packets - Character Server

**CH_ENTER (0x0065)** - Login al char server:
```
[2 bytes] header = 0x0065
[4 bytes] account_id (u32)
[4 bytes] login_id1 (u32)
[4 bytes] login_id2 (u32)
[2 bytes] unknown (u16, default 0)
[1 byte]  sex (u8)
```
Total: 17 bytes

**IMPORTANTE - Account ID raw**: Inmediatamente despues de conectar al char server, este envia 4 bytes raw con el account_id SIN header de packet. El cliente los lee antes de procesar cualquier otro packet.

**HC_ACCEPT_ENTER (0x082D)** - Char server acepta conexion:
```
[2 bytes]  header = 0x082D
[2 bytes]  unknown (u16, siempre 29)
[1 byte]   normal_slot_count (u8)
[1 byte]   vip_slot_count (u8)
[1 byte]   billing_slot_count (u8)
[1 byte]   producible_slot_count (u8)
[1 byte]   valid_slot (u8)
[20 bytes]  unused (padding)
```
Total: 29 bytes

**Packets que el char server envia automaticamente despues de 0x082D** (el cliente los registra como noop - los acepta pero ignora):

| Header | Nombre | Descripcion |
|--------|--------|-------------|
| 0x006B | CharacterListPacket | Lista alternativa de chars (ignorada, se usa 0x0B72) |
| 0x09A0 | CharacterSlotPagePacket | Info de paginas de slots |
| 0x020D | CharacterBanListPacket | Lista de bans de personajes |
| 0x08B9 | LoginPincodePacket | Pincode state (pincode_seed + account_id + state) |
| 0x0B18 | Packet0b18 | Desconocido, posiblemente inventario |

**CH_SELECT_CHAR (0x09A1)** - Pedir lista de personajes:
```
[2 bytes] header = 0x09A1
```
(packet vacio, solo header)

**HC_ACCEPT_MAKECHAR (0x0B72)** - Lista de personajes (variable-length):
```
[2 bytes] header = 0x0B72
[2 bytes] packet_len
[remaining] CharacterInformation[] (ver estructura abajo)
```

**CH_SELECT_CHAR (0x0066)** - Seleccionar personaje:
```
[2 bytes] header = 0x0066
[1 byte]  selected_slot (u8, 0-based)
```

**HC_NOTIFY_ZONESVR (0x0AC5)** - Personaje seleccionado con exito:
```
[2 bytes]   header = 0x0AC5
[4 bytes]   character_id (u32)
[16 bytes]  map_name (string, null-padded a 16 bytes, ej: "prontera.gat")
[4 bytes]   map_server_ip (u32, IPv4)
[2 bytes]   map_server_port (u16)
[128 bytes] unknown (padding)
```
Total: 156 bytes

**CH_MAKE_CHAR (0x0A39)** - Crear personaje:
```
[2 bytes]  header = 0x0A39
[24 bytes] name (string, null-padded a 24 bytes)
[1 byte]   slot (u8)
[2 bytes]  hair_color (u16)
[2 bytes]  hair_style (u16)
[2 bytes]  start_job (u16)
[2 bytes]  unknown (u16, default 0)
[1 byte]   sex (u8)
```
Total: 36 bytes

**HC_ACCEPT_MAKECHAR (0x0B6F)** - Personaje creado exitosamente:
```
[2 bytes] header = 0x0B6F
[...] CharacterInformation (misma estructura que en la lista)
```

**CH_DELETE_CHAR (0x01FB)** - Eliminar personaje:
```
[2 bytes]  header = 0x01FB
[4 bytes]  character_id (u32)
[40 bytes] email (string, null-padded a 40 bytes, o date of birth)
[10 bytes] unknown (padding)
```
Total: 56 bytes

**Keepalive (0x0187)** - Cada 10 segundos:
```
[2 bytes] header = 0x0187
[4 bytes] account_id (u32, siempre 0)
```

### 0.5 Estructura CharacterInformation

Cada personaje en la lista usa esta estructura de tamano fijo:
```
[4 bytes]  character_id (u32)
[8 bytes]  experience (i64)
[4 bytes]  money/zeny (i32)
[8 bytes]  job_experience (i64)
[4 bytes]  job_level (i32)
[4 bytes]  body_state (i32)
[4 bytes]  health_state (i32)
[4 bytes]  effect_state (i32)
[4 bytes]  virtue (i32)
[4 bytes]  honor (i32)
[2 bytes]  stat_points (i16)
[8 bytes]  health_points (i64)
[8 bytes]  maximum_health_points (i64)
[8 bytes]  spell_points (i64)
[8 bytes]  maximum_spell_points (i64)
[2 bytes]  movement_speed (i16)
[2 bytes]  job (i16)
[2 bytes]  head (i16)
[2 bytes]  body (i16)
[2 bytes]  weapon (i16)
[2 bytes]  base_level (i16)
[2 bytes]  sp_point (i16)
[2 bytes]  accessory (i16)
[2 bytes]  shield (i16)
[2 bytes]  accessory2 (i16)
[2 bytes]  accessory3 (i16)
[2 bytes]  head_palette (i16)
[2 bytes]  body_palette (i16)
[24 bytes] name (string, null-padded a 24 bytes)
[1 byte]   strength (u8)
[1 byte]   agility (u8)
[1 byte]   vitality (u8)
[1 byte]   intelligence (u8)
[1 byte]   dexterity (u8)
[1 byte]   luck (u8)
[1 byte]   character_number/slot (u8)
[1 byte]   hair_color (u8)
[2 bytes]  b_is_changed_char (i16)
[16 bytes] map_name (string, null-padded a 16 bytes)
[4 bytes]  deletion_reverse_date (i32)
[4 bytes]  robe_palette (i32)
[4 bytes]  character_slot_change_count (i32)
[4 bytes]  character_name_change_count (i32)
[1 byte]   sex (u8)
```

**NOTA**: Esta estructura usa `i64` para HP/SP/EXP (packet version 20220406 usa 64-bit para estos campos). rAthena debe enviar estos como 8 bytes, no 4.

### 0.6 Detalle de Packets - Map Server

**CZ_ENTER (0x0436)** - Login al map server:
```
[2 bytes] header = 0x0436
[4 bytes] account_id (u32)
[4 bytes] character_id (u32)
[4 bytes] login_id1 (u32)
[4 bytes] client_tick (u32, inicialmente 100)
[1 byte]  sex (u8)
[4 bytes] unknown (default 0)
```
Total: 19 bytes

**ZC_ACCEPT_ENTER (0x02EB)** - Map server acepta:
```
[2 bytes] header = 0x02EB
[4 bytes] client_tick (u32)
[3 bytes] position (WorldPosition, packed x/y/dir)
[2 bytes] ignored (siempre [5, 5])
[2 bytes] font (u16)
```
Total: 13 bytes

El campo `position` de 3 bytes usa el formato estandar de RO:
- bits[0-9]: x coordinate (10 bits)
- bits[10-19]: y coordinate (10 bits)
- bits[20-23]: direction (4 bits)

**CZ_NOTIFY_ACTORINIT (0x007D)** - Mapa cargado:
```
[2 bytes] header = 0x007D
```
(packet vacio, solo header. El cliente lo envia despues de cargar los recursos del mapa)

**Ping del map server (0x0B1D)** - Servidor envia ping periodicamente:
```
[2 bytes] header = 0x0B1D
```
(packet vacio, el cliente lo acepta silenciosamente)

### 0.7 Configuracion Requerida en rAthena

#### login_athena.conf
```conf
// Puerto del login server
login_port: 6900

// DEBE coincidir con <version> en sclientinfo.xml
client_version_to_connect: 55

// NO usar si sclientinfo tiene <passwordencrypt> o <passwordencrypt2>
use_MD5_passwords: no

// Packet version
packet_ver: 20220406
```

#### char_athena.conf
```conf
// Puerto del char server
char_port: 6121

// Nombre del servidor (max 20 bytes, coincide con server_name en 0x0AC4)
server_name: AnimaRO

// Slots de personajes
chars_per_account: 9

// Pincode (el cliente ignora pincode, pero el server puede enviarlo)
pincode_enabled: no
```

#### map_athena.conf
```conf
// Puerto del map server
map_port: 5121
```

#### packet_db.txt / src/common/mmo.hpp
```conf
// Packet version para cliente Korangar
PACKETVER 20220406
```

### 0.8 Packets del Char Server que el Cliente Acepta como NOOP

El cliente registra estos packets como "noop" (los acepta del stream TCP pero no hace nada con ellos). rAthena los envia por defecto asi que NO hay que desactivarlos:

| Header | Nombre | Descripcion |
|--------|--------|-------------|
| 0x006B | CharacterListPacket | Lista antigua de chars (se usa 0x0B72 en su lugar) |
| 0x09A0 | CharacterSlotPagePacket | Paginas de slots (page_quantity: u32) |
| 0x020D | CharacterBanListPacket | Lista de chars baneados |
| 0x08B9 | LoginPincodePacket | Estado del pincode |
| 0x0B18 | Packet0b18 | Desconocido (2 bytes unknown) |
| 0x0B1D | MapServerPingPacket | Ping del map server |

Estos packets DEBEN ser enviados por el servidor porque el cliente los espera en el stream TCP. Si no estan presentes, no pasa nada (el cliente no los necesita), pero si el servidor los envia con formato incorrecto, el cliente se desincroniza.

### 0.9 Posibles Problemas de Conexion

**0.9.1 auth_token de 17 bytes**: En 0x0AC4, despues del campo `sex`, hay un `auth_token` de exactamente 17 bytes. Si rAthena no lo envia o envia un tamano diferente, el parseo del array de character servers se corrompe.

**0.9.2 Account ID raw en char server**: Al conectar al char server, los primeros 4 bytes que llegan NO son un packet normal. Son el account_id en raw sin header. El cliente los lee especialmente antes de entrar al loop de packets.

**0.9.3 HP/SP/EXP como i64**: Para packet version 20220406, `CharacterInformation` usa campos de 8 bytes (i64) para health_points, maximum_health_points, spell_points, maximum_spell_points, experience, y job_experience. Si rAthena envia estos como i32 (4 bytes), toda la estructura se desalinea.

**0.9.4 CharacterServerInformation de 160 bytes**: Cada entrada de servidor en 0x0AC4 tiene un campo `unknown` de 128 bytes al final. Total por entrada = 160 bytes.

**0.9.5 CharacterSelectionSuccessPacket (0x0AC5) de 156 bytes**: Tiene un campo `unknown` de 128 bytes al final. Si rAthena usa la version vieja (0x0071, sin este padding), el cliente no lo reconoce.

**0.9.6 MapServerLoginPacket es 0x0436, NO 0x0064**: Para version 20220406, el header del login al map server es 0x0436, no el clasico 0x0064. rAthena debe tener esto en su packet_db.

---

## 1. Chat / Social

### 1.1 Whisper (Mensaje privado)

**Cliente envia** `CZ_WHISPER` (0x0096) - longitud variable:
```
[2 bytes] header = 0x0096
[2 bytes] packet_len (total)
[24 bytes] receiver_name (string, null-padded a 24 bytes)
[remaining] message (string, longitud = packet_len - 28)
```

**Servidor debe responder con** `ZC_ACK_WHISPER` (0x0098) - 3 bytes fijos:
```
[2 bytes] header = 0x0098
[1 byte]  result (u8):
          0 = exito
          1 = jugador no encontrado
          2 = bloqueado
```

**Al jugador destino, enviar** `ZC_WHISPER` (0x0097) - longitud variable:
```
[2 bytes] header = 0x0097
[2 bytes] packet_len (total)
[24 bytes] sender_name (string, null-padded a 24 bytes)
[remaining] message (string, longitud = packet_len - 28)
```

### 1.2 Party Chat

**Cliente envia** `CZ_REQUEST_CHAT_PARTY` (0x0108) - longitud variable:
```
[2 bytes] header = 0x0108
[2 bytes] packet_len (total)
[remaining] message (string, formato "NombreJugador : texto")
```

**Servidor broadcast** `ZC_NOTIFY_CHAT_PARTY` (0x0109) - longitud variable:
```
[2 bytes] header = 0x0109
[2 bytes] packet_len (total)
[4 bytes] account_id (u32)
[remaining] message (string, formato "NombreJugador : texto")
```

**IMPORTANTE**: El cliente parsea el sender_name haciendo `split(" : ")` sobre el message. El formato del string DEBE ser `"NombreJugador : texto"` con espacios alrededor del `:`.

### 1.3 Guild Chat

**Cliente envia** `CZ_GUILD_CHAT` (0x017E) - longitud variable:
```
[2 bytes] header = 0x017E
[2 bytes] packet_len (total)
[remaining] message (string, formato "NombreJugador : texto")
```

**Servidor broadcast** `ZC_GUILD_CHAT` (0x017F) - longitud variable:
```
[2 bytes] header = 0x017F
[2 bytes] packet_len (total)
[remaining] message (string)
```

### 1.4 Emotes

**Cliente envia** `CZ_REQ_EMOTION` (0x00BF) - 3 bytes fijos:
```
[2 bytes] header = 0x00BF
[1 byte]  emotion (u8, rango 0-99)
```

**Servidor debe broadcast** `ZC_EMOTION` (0x00C0) - 7 bytes fijos:
```
[2 bytes] header = 0x00C0
[4 bytes] entity_id (u32)
[1 byte]  emotion (u8)
```

---

## 2. Trade (Intercambio)

### Flujo completo

| Paso | Quien | Packet | Header | Bytes |
|------|-------|--------|--------|-------|
| 1 | C->S | `CZ_REQ_TRADE` | 0x00E4 | 6 |
| 2 | S->C | `ZC_REQ_TRADE` | 0x01F4 | 8 |
| 3 | C->S | `CZ_ACK_TRADE` | 0x00E6 | 3 |
| 4 | S->C | `ZC_ACK_TRADE` | 0x00E7 | 3 |
| 5 | C->S | `CZ_ADD_EXCHANGE_ITEM` | 0x00E8 | 8 |
| 6 | S->C | `ZC_ADD_EXCHANGE_ITEM` | 0x00E9 | 21 |
| 7 | C->S | `CZ_CONCLUDE_EXCHANGE_ITEM` | 0x00EB | 2 |
| 8 | S->C | `ZC_CONCLUDE_EXCHANGE_ITEM` | 0x00EC | 3 |
| 9 | C->S | `CZ_EXEC_EXCHANGE_ITEM` | 0x00EF | 2 |
| 10 | S->C | `ZC_EXEC_EXCHANGE_ITEM` | 0x00F0 | 3 |
| Cancel | C->S | `CZ_CANCEL_EXCHANGE_ITEM` | 0x00ED | 2 |
| Cancelado | S->C | `ZC_CANCEL_EXCHANGE_ITEM` | 0x00EE | 2 |

### Detalle de cada packet

**CZ_REQ_TRADE (0x00E4)** - Solicitar trade:
```
[2 bytes] header = 0x00E4
[4 bytes] entity_id (u32)
```

**ZC_REQ_TRADE (0x01F4)** - Notificar solicitud al otro jugador:
```
[2 bytes] header = 0x01F4
[4 bytes] entity_id del solicitante (u32)
[2 bytes] base_level del solicitante (u16)
```
El cliente muestra una ventana con Accept/Reject usando estos datos.

**CZ_ACK_TRADE (0x00E6)** - Respuesta del jugador:
```
[2 bytes] header = 0x00E6
[1 byte]  response (u8): 3 = aceptar, 4 = rechazar
```

**ZC_ACK_TRADE (0x00E7)** - Resultado a ambos jugadores:
```
[2 bytes] header = 0x00E7
[1 byte]  response (u8):
          0 = too far away
          1 = character does not exist
          2 = trade refused
          3 = trade accepted (cliente abre ventana de trade)
          4 = cancelled
```

**CZ_ADD_EXCHANGE_ITEM (0x00E8)** - Agregar item:
```
[2 bytes] header = 0x00E8
[2 bytes] inventory_index (u16)
[4 bytes] amount (u32)
```
Si `index == 0`, se interpreta como Zeny.

**ZC_ADD_EXCHANGE_ITEM (0x00E9)** - Notificar item agregado al partner:
```
[2 bytes] header = 0x00E9
[4 bytes] amount (u32)
[4 bytes] item_id (u32) -- 0 si es zeny
[1 byte]  is_identified (u8)
[1 byte]  is_broken (u8)
[1 byte]  refinement_level (u8)
[8 bytes] card_slots [u16; 4] -- 4 slots de 2 bytes cada uno
```
**CRITICO**: Si `item_id == 0`, el cliente interpreta `amount` como Zeny del partner. Si `item_id != 0`, lo agrega como item.

**CZ_CONCLUDE_EXCHANGE_ITEM (0x00EB)** - Lock:
```
[2 bytes] header = 0x00EB
```

**ZC_CONCLUDE_EXCHANGE_ITEM (0x00EC)** - Lock status:
```
[2 bytes] header = 0x00EC
[1 byte]  who (u8): 0 = self locked, 1 = partner locked
```

**CZ_EXEC_EXCHANGE_ITEM (0x00EF)** - Completar:
```
[2 bytes] header = 0x00EF
```

**ZC_EXEC_EXCHANGE_ITEM (0x00F0)** - Resultado final:
```
[2 bytes] header = 0x00F0
[1 byte]  result (u8): 0 = exito, 1 = fallo
```

**CZ_CANCEL_EXCHANGE_ITEM (0x00ED)** - Cancelar:
```
[2 bytes] header = 0x00ED
```

**ZC_CANCEL_EXCHANGE_ITEM (0x00EE)** - Cancelado:
```
[2 bytes] header = 0x00EE
```

---

## 3. Minimap

**No requiere cambios en el servidor.** Es puramente client-side.

---

## 4. Pet System

### 4.1 Comando de pet (alimentar)

**Cliente envia** `CZ_COMMAND_PET` (0x01A1) - 3 bytes fijos:
```
[2 bytes] header = 0x01A1
[1 byte]  command_type (u8): 1 = feed
```

### 4.2 Packets que el servidor debe enviar

**ZC_PET_ACT (0x01A4)** - Resultado de alimentar:
```
[2 bytes] header = 0x01A4
[1 byte]  success (u8): 0 = fallo, 1 = exito
[4 bytes] food_item_id (u32)
```

**ZC_PROPERTY_PET (0x01A2)** - Info del pet:
```
[2 bytes] header = 0x01A2
[24 bytes] name (string, null-padded a 24 bytes)
[1 byte]  renamed (u8): 0 = no, 1 = si
[2 bytes] level (u16)
[2 bytes] loyalty (u16, 0-1000)
[2 bytes] accessory (u16, item ID del accesorio)
[2 bytes] class_id (u16, monster ID del pet)
```
Total: 35 bytes

**ZC_PETEGG_LIST (0x01A6)** - Lista de huevos incubables:
```
[2 bytes] header = 0x01A6
[2 bytes] packet_len (total)
[remaining] array de inventory_index (u16 cada uno)
```
Cantidad de huevos = (packet_len - 4) / 2

**ZC_PET_CATCH_RESULT (0x01A0)** - Resultado captura:
```
[2 bytes] header = 0x01A0
[1 byte]  success (u8): 0 = fallo, != 0 = exito
```

**ZC_PET_EMOTION (0x01AA)** - Emocion del pet:
```
[2 bytes] header = 0x01AA
[4 bytes] entity_id (u32)
[4 bytes] emotion (u32)
```

---

## 5. Homunculus System

### 5.1 Comando de homunculus (alimentar)

**Cliente envia** `CZ_HOMUNCULUS_COMMAND` (0x022D) - 4 bytes fijos:
```
[2 bytes] header = 0x022D
[2 bytes] action_type (u16): 1 = feed
```

### 5.2 Packets que el servidor debe enviar

**ZC_PROPERTY_HOMUN (0x022E)** - Info del homunculus:
```
[2 bytes] header = 0x022E
[24 bytes] name (string, null-padded a 24 bytes)
[1 byte]  renamed (u8)
[2 bytes] level (u16)
[2 bytes] loyalty (u16, 0-1000)
[2 bytes] feed/hunger (u16, 0-100)
[2 bytes] accessory (u16)
[2 bytes] class_id (u16)
[4 bytes] hp (u32)
[4 bytes] max_hp (u32)
[2 bytes] sp (u16)
[2 bytes] max_sp (u16)
[2 bytes] atk (u16)
[2 bytes] matk (u16)
[2 bytes] hit (u16)
[2 bytes] critical (u16)
[2 bytes] defense (u16)
[2 bytes] mdefense (u16)
[2 bytes] flee (u16)
[2 bytes] aspd (u16)
[2 bytes] str (u16)
[2 bytes] agi (u16)
[2 bytes] vit (u16)
[2 bytes] int (u16)
[2 bytes] dex (u16)
[2 bytes] luk (u16)
```
Total: 79 bytes. El cliente solo usa name, level, loyalty, feed, class_id, hp, max_hp, sp, max_sp. Los demas stats (atk, matk, etc.) se ignoran pero DEBEN estar presentes porque el cliente lee bytes secuencialmente.

**ZC_HOMMENU_FLAG (0x022F)** - Resultado de alimentar:
```
[2 bytes] header = 0x022F
[1 byte]  success (u8): 0 = fallo, != 0 = exito
[4 bytes] food_item_id (u32)
```

**ZC_HOMUN_ALIVE (0x0230)** - Estado alive/dead:
```
[2 bytes] header = 0x0230
[1 byte]  alive (u8): 0 = muerto, != 0 = vivo
```

---

## 6. Tablas Resumen

### Packets que el cliente ENVIA - Login/Char/Map Server

| Header | Server | Nombre | Tam | Descripcion |
|--------|--------|--------|-----|-------------|
| 0x0064 | Login | CA_LOGIN | 55 | Autenticacion (username + password) |
| 0x0200 | Login | Keepalive | 26 | Ping cada 58s |
| 0x0065 | Char | CH_ENTER | 17 | Login al char server |
| 0x09A1 | Char | CH_SELECT_CHAR_REQ | 2 | Pedir lista personajes |
| 0x0066 | Char | CH_SELECT_CHAR | 3 | Seleccionar personaje (slot) |
| 0x0A39 | Char | CH_MAKE_CHAR | 36 | Crear personaje |
| 0x01FB | Char | CH_DELETE_CHAR | 56 | Eliminar personaje |
| 0x0187 | Char | Keepalive | 6 | Ping cada 10s |
| 0x0436 | Map | CZ_ENTER | 19 | Login al map server |
| 0x007D | Map | CZ_NOTIFY_ACTORINIT | 2 | Mapa cargado |

### Packets que el servidor ENVIA - Login/Char/Map Server

| Header | Server | Nombre | Tam | Descripcion |
|--------|--------|--------|-----|-------------|
| 0x0AC4 | Login | AC_ACCEPT_LOGIN | var | Login OK + lista char servers |
| 0x083E | Login | AC_REFUSE_LOGIN | 3 | Login fallido (reason) |
| 0x0081 | Login/Char | SC_NOTIFY_BAN | 3 | Desconexion/ban |
| (raw) | Char | (account_id) | 4 | 4 bytes raw sin header |
| 0x082D | Char | HC_ACCEPT_ENTER | 29 | Char server OK (slots info) |
| 0x006B | Char | CharacterListPacket | var | Lista chars (noop) |
| 0x09A0 | Char | CharacterSlotPagePacket | 6 | Slot pages (noop) |
| 0x020D | Char | CharacterBanListPacket | var | Ban list (noop) |
| 0x08B9 | Char | LoginPincodePacket | 10 | Pincode (noop) |
| 0x0B18 | Char | Packet0b18 | 4 | Unknown (noop) |
| 0x0B72 | Char | HC_CHAR_LIST | var | Lista de personajes |
| 0x0AC5 | Char | HC_NOTIFY_ZONESVR | 156 | Map server info + char_id |
| 0x0B6F | Char | HC_ACCEPT_MAKECHAR | var | Personaje creado |
| 0x006E | Char | HC_REFUSE_MAKECHAR | 3 | Creacion fallida |
| 0x006C | Char | HC_REFUSE_ENTER | 3 | Seleccion fallida |
| 0x0840 | Char | MapServerUnavailablePacket | var | Map server no disponible |
| 0x02EB | Map | ZC_ACCEPT_ENTER | 13 | Map server OK (pos + tick) |
| 0x0B1D | Map | MapServerPingPacket | 2 | Ping (noop) |

### Packets que el cliente ENVIA - Gameplay (Map Server)

| Header | Nombre | Tam | Descripcion |
|--------|--------|-----|-------------|
| 0x0096 | CZ_WHISPER | var | Enviar whisper (24b name + msg) |
| 0x0108 | CZ_REQUEST_CHAT_PARTY | var | Chat de party |
| 0x017E | CZ_GUILD_CHAT | var | Chat de guild |
| 0x00BF | CZ_REQ_EMOTION | 3 | Emote (1 byte emotion) |
| 0x00E4 | CZ_REQ_TRADE | 6 | Solicitar trade (4b entity_id) |
| 0x00E6 | CZ_ACK_TRADE | 3 | Responder trade (3=accept, 4=reject) |
| 0x00E8 | CZ_ADD_EXCHANGE_ITEM | 8 | Agregar item (2b index + 4b amount) |
| 0x00EB | CZ_CONCLUDE_EXCHANGE_ITEM | 2 | Lock trade |
| 0x00EF | CZ_EXEC_EXCHANGE_ITEM | 2 | Completar trade |
| 0x00ED | CZ_CANCEL_EXCHANGE_ITEM | 2 | Cancelar trade |
| 0x01A1 | CZ_COMMAND_PET | 3 | Comando pet (1=feed) |
| 0x022D | CZ_HOMUNCULUS_COMMAND | 4 | Comando hom (1=feed) |

### Packets que el cliente ESPERA - Gameplay (Map Server)

| Header | Nombre | Tam | Descripcion |
|--------|--------|-----|-------------|
| 0x0097 | ZC_WHISPER | var | Whisper recibido |
| 0x0098 | ZC_ACK_WHISPER | 3 | Resultado whisper (0=ok, 1=not found, 2=blocked) |
| 0x0109 | ZC_NOTIFY_CHAT_PARTY | var | Party chat (4b account_id + msg) |
| 0x017F | ZC_GUILD_CHAT | var | Guild chat |
| 0x00C0 | ZC_EMOTION | 7 | Emote display (4b entity_id + 1b emotion) |
| 0x01F4 | ZC_REQ_TRADE | 8 | Trade request (4b entity_id + 2b level) |
| 0x00E7 | ZC_ACK_TRADE | 3 | Trade result (0-4) |
| 0x00E9 | ZC_ADD_EXCHANGE_ITEM | 21 | Item agregado al trade |
| 0x00EC | ZC_CONCLUDE_EXCHANGE_ITEM | 3 | Lock status (0=self, 1=partner) |
| 0x00F0 | ZC_EXEC_EXCHANGE_ITEM | 3 | Resultado trade (0=ok, 1=fail) |
| 0x00EE | ZC_CANCEL_EXCHANGE_ITEM | 2 | Trade cancelado |
| 0x01A2 | ZC_PROPERTY_PET | 35 | Info pet |
| 0x01A4 | ZC_PET_ACT | 7 | Resultado feed pet |
| 0x01A6 | ZC_PETEGG_LIST | var | Lista huevos |
| 0x01A0 | ZC_PET_CATCH_RESULT | 3 | Resultado captura |
| 0x01AA | ZC_PET_EMOTION | 10 | Emocion pet |
| 0x022E | ZC_PROPERTY_HOMUN | 79 | Info homunculus (todos los stats) |
| 0x022F | ZC_HOMMENU_FLAG | 7 | Resultado feed hom |
| 0x0230 | ZC_HOMUN_ALIVE | 3 | Alive state hom |

---

## 7. Checklist de Verificacion en rAthena

### 7.0 Login / Char / Map Server

- [ ] `PACKETVER` configurado a `20220406` en `src/common/mmo.hpp` o equivalent
- [ ] `client_version_to_connect` en `login_athena.conf` coincide con `<version>` en sclientinfo.xml
- [ ] `use_MD5_passwords: no` si sclientinfo NO tiene `<passwordencrypt>`
- [ ] Login server envia 0x0AC4 con `auth_token` de 17 bytes y CharacterServerInformation de 160 bytes cada una
- [ ] Char server envia 4 bytes raw de account_id al conectar (antes de cualquier packet)
- [ ] Char server envia 0x082D (29 bytes) seguido de los packets noop (0x006B, 0x09A0, 0x020D, 0x08B9, 0x0B18)
- [ ] Char server envia lista de personajes via 0x0B72 (NO 0x006B)
- [ ] `CharacterInformation` usa i64 (8 bytes) para HP, MaxHP, SP, MaxSP, EXP, JobEXP
- [ ] Char server responde a 0x0066 con 0x0AC5 (156 bytes, incluye 128 bytes padding)
- [ ] Map server acepta login via 0x0436 (NO 0x0064 que es para login server)
- [ ] Map server responde con 0x02EB (13 bytes, posicion packed en 3 bytes)
- [ ] Map server acepta 0x007D como notificacion de mapa cargado
- [ ] `pincode_enabled: no` en char_athena.conf (cliente no soporta pincode interactivo)

### 7.1 Packet Database

Verificar en `conf/packet_db.txt` o `src/map/packets.hpp` que para packet version `20220406`:

- [ ] Todos los headers de la tabla "Cliente ENVIA" estan mapeados con sus tamanios correctos
- [ ] Los packets variable-length (0x0096, 0x0108, 0x017E) tienen tamanio `-1` en packet_db
- [ ] Los packets fijos tienen el tamanio correcto

### 7.2 Chat (src/map/clif.cpp)

- [ ] `clif_wis_message` envia 0x0097 con name de 24 bytes null-padded
- [ ] `clif_wis_end` envia 0x0098 con result correcto
- [ ] `clif_party_message` envia 0x0109 con account_id + message
- [ ] `clif_guild_message` envia 0x017F con message
- [ ] `clif_emotion` envia 0x00C0 con entity_id + emotion
- [ ] El handler de 0x00BF valida emotion range 0-99

### 7.3 Trade (src/map/trade.cpp)

- [ ] `clif_traderequest` envia 0x01F4 con entity_id (4b) + base_level (2b)
- [ ] `clif_traderesponse` envia 0x00E7 con response correcto
- [ ] `clif_tradeadditem` envia 0x00E9 con formato exacto (21 bytes)
- [ ] Zeny se envia como item_id=0 en 0x00E9
- [ ] `clif_tradeconclude` envia 0x00EC con who (0=self, 1=partner)
- [ ] `clif_traderesult` envia 0x00F0 con result (0=ok, 1=fail)
- [ ] `clif_tradecancel` envia 0x00EE
- [ ] No hay restricciones custom que bloqueen el trade

### 7.4 Pet (src/map/pet.cpp, conf/battle/pet.conf)

- [ ] Sistema de pets habilitado en pet.conf
- [ ] `pet_hungry_delay` configurado
- [ ] Handler de 0x01A1 parsea command_type (1=feed)
- [ ] `clif_pet_food` envia 0x01A4 (success + food_item_id)
- [ ] `clif_send_petdata` envia 0x01A2 (35 bytes exactos)
- [ ] `clif_pet_eggs` envia 0x01A6 (variable-length)
- [ ] `clif_pet_catch_result` envia 0x01A0
- [ ] `clif_pet_emotion` envia 0x01AA (entity_id u32 + emotion u32)

### 7.5 Homunculus (src/map/homunculus.cpp)

- [ ] Sistema de homunculus habilitado
- [ ] Handler de 0x022D parsea action_type como u16 (2 bytes, no 1)
- [ ] `clif_hominfo` envia 0x022E con los 79 bytes completos
- [ ] `clif_hom_food` envia 0x022F (success + food_item_id)
- [ ] `clif_homalive` envia 0x0230

### 7.6 Emotes

- [ ] Nativos en rAthena, verificar que no haya restriccion custom
- [ ] El broadcast de 0x00C0 incluye el entity_id del emisor

---

## 8. Posibles Problemas

### 8.1 Packet Version Mismatch
Si rAthena usa un packet_db diferente para 20220406, algunos headers pueden estar remapeados (packet obfuscation). Verificar que el shuffle table coincida.

### 8.2 CZ_HOMUNCULUS_COMMAND action_type
El cliente envia `action_type` como `u16` (2 bytes), no `u8`. rAthena debe leer 2 bytes para este campo.

### 8.3 ZC_PROPERTY_HOMUN tamano fijo
El cliente lee 79 bytes secuencialmente. Si rAthena envia menos bytes, el cliente se desincroniza y los packets siguientes se corrompen.

### 8.4 Trade item_id==0 para Zeny
Cuando el servidor envia `ZC_ADD_EXCHANGE_ITEM` (0x00E9) con `item_id=0`, el cliente lo interpreta como Zeny. `amount` se convierte en la cantidad de Zeny del partner.

### 8.5 Party Chat formato de string
El mensaje de party chat DEBE tener formato `"NombreJugador : texto"` (con espacios alrededor de `:`). El cliente parsea el sender_name haciendo `split(" : ")`.

### 8.6 Variable-length packets
Los packets con `packet_len` en bytes 2-3 son variable-length. rAthena debe leer `packet_len` para saber cuantos bytes consumir del stream TCP.
