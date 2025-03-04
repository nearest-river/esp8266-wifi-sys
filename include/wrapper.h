// Licensed under the Apache License, Version 2.0 (the "License");
// See LICENSE-THIRD-PARTY for details.
//
// Modified by esp8266-wifi-sys, 2025
//
// Modifications:
//    Removed `struct timeval {...}`
//    Removed esp32s2, esp32c6, esp32h2 specific code.
//

#include <stdint.h>

typedef int _lock_t;

#define SOC_COEX_HW_PTI 1

typedef uint32_t        TickType_t;
typedef uint32_t        UBaseType_t;
typedef int32_t         BaseType_t;

typedef void*           QueueHandle_t;

typedef void*           esp_netif_t;
typedef void*           esp_netif_inherent_config_t;

struct ets_timer
{
  struct timer_adpt *next;
  uint32_t expire;
  uint32_t period;
  void (*func)(void *priv);
  void *priv;
};


#include "esp_private/wifi.h"
#include "esp_private/wifi_os_adapter.h"
#include "esp_wpa.h"
#include "esp_phy_init.h"
#include "phy.h"
#include "esp_timer.h"
#include "esp_eap_client.h"


#include "esp_now.h"
