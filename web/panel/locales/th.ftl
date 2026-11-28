## Frame

panel-brand = IMPULSE · รีโมท
panel-no-game = ไม่มีเกมที่เปิดอยู่
dock-no-run = ไม่ได้เล่นอยู่
drawer-close = ปิด
search-clear = ล้างการค้นหา
link-open = ออนไลน์
link-connecting = กำลังเชื่อมต่อ
link-lost = ออฟไลน์

## Connection

connection-title = การเชื่อมต่อ
connection-close = ปิดแผงการเชื่อมต่อ
connection-pairing = หากต้องการจับคู่อุปกรณ์อื่น ให้กดปุ่มโทรศัพท์ในแถบชื่อของ Impulse แล้วสแกนคิวอาร์โค้ดด้วยอุปกรณ์นั้น
connection-follows = ภาษาและสีเน้นจะตามค่าของ Impulse

## Placeholders

placeholder-refused = จับคู่ไม่สำเร็จ
placeholder-refused-body = รหัสถูกใช้แล้ว หมดอายุ หรือลองบ่อยเกินไป สแกนคิวอาร์โค้ดใน Impulse อีกครั้ง
placeholder-outdated = ต้องโหลดใหม่
placeholder-outdated-body = Impulse อัปเดตแล้ว: หน้านี้เก่ากว่า
placeholder-reload = โหลดใหม่
placeholder-offline = ไม่ได้เชื่อมต่อ
placeholder-offline-body = เปิด Impulse บนพีซี กดปุ่มโทรศัพท์ในแถบชื่อ เปิดการเข้าถึงจากโทรศัพท์ แล้วสแกนคิวอาร์โค้ด อุปกรณ์นี้ต้องอยู่ในเครือข่ายเดียวกัน
placeholder-connecting = กำลังเชื่อมต่อกับ Impulse
placeholder-pick = เลือกเกม
placeholder-pick-body = ไม่มีเกมที่เปิดอยู่ เปิดคลังเกมแล้วเลือกหนึ่งเกม
placeholder-browse = เปิดคลังเกม

## Library

library-title = คลังเกม
library-close = ปิดคลังเกม
library-count = { $count ->
    [one] { $count } เกม
   *[other] { $count } เกม
}
library-search = ค้นหาเกม
library-playing = กำลังเล่น
library-favorites = รายการโปรด
library-all = เกมทั้งหมด
library-open = เปิด
library-no-match = ไม่พบเกมที่ตรงกับ “{ $query }”
library-empty = คลังเกมว่างเปล่า

## Game

game-start = เริ่มเกมและเทรนเนอร์
game-stop = หยุดเทรนเนอร์
game-favorite = เพิ่มในรายการโปรด
game-unfavorite = เอาออกจากรายการโปรด
run-idle = ไม่ได้ทำงาน
run-starting = กำลังเริ่ม
run-live = เทรนเนอร์เปิดอยู่
run-stopping = กำลังหยุด
run-failed = เริ่มไม่สำเร็จ

## Trainer

sheet-none = ยังไม่มีเทรนเนอร์
sheet-none-body = เกมนี้ยังไม่มีเทรนเนอร์ Impulse จะแสดงป้ายในคลังเกมทันทีที่มี
sheet-pending = กำลังโหลดรายการโกง
sheet-failed = ยังไม่ได้ดาวน์โหลดรายการโกง
sheet-search = ค้นหาม็อด
sheet-start-first = เริ่มเกมก่อนจึงจะใช้ม็อดได้
sheet-no-match = ไม่พบม็อดที่ตรงกับ “{ $query }”
sheet-matches = { $count ->
    [one] พบ { $count } รายการ
   *[other] พบ { $count } รายการ
}
sheet-end = จบ · { $count ->
    [one] { $count } ม็อด
   *[other] { $count } ม็อด
}
section-mods = { $count ->
    [one] { $count } ม็อด
   *[other] { $count } ม็อด
}
section-mods-on = { $count ->
    [one] { $count } ม็อด
   *[other] { $count } ม็อด
} · เปิด { $on }/{ $toggles }
cheat-pin = ปักหมุด { $name }
cheat-unpin = เลิกปักหมุด { $name }

## Collections

actions-reset = รีเซ็ต
actions-collect = ชุด
actions-forget = ลบชุด { $name }
collect-title = ชุดใหม่
collect-tip = บันทึกม็อดที่เปิดอยู่ตอนนี้เป็นชุด
collect-name = ชื่อ
collect-cancel = ยกเลิก
collect-save = บันทึก
collect-close = ปิด

## Controls

control-press = เปิดใช้
control-apply = ใช้
control-value = { $name }: ค่า
control-decrease = ลด { $name }
control-increase = เพิ่ม { $name }
control-no-options = ไม่มีตัวเลือก
