use anchor_lang::prelude::*;

declare_id!("HEa3C2KBsxhmF3HKBxCVfj5e6yryAWrmRgNN7AE96BV1");

#[program]
pub mod smart_ticket {
    use super::*;

    // ==================== 🎟️ CREAR EVENTO ====================
    pub fn crear_evento(
        ctx: Context<CrearEvento>,
        evento_id: u64,
        nombre: String,
        fecha: i64,
        ubicacion_hash: [u8; 32],
        precio_base: u64,
        max_capacidad: u32,
    ) -> Result<()> {
        let evento = &mut ctx.accounts.evento;
        evento.id = evento_id;
        evento.nombre = nombre;
        evento.fecha = fecha;
        evento.ubicacion_hash = ubicacion_hash;
        evento.precio_base = precio_base;
        evento.max_capacidad = max_capacidad;
        evento.tickets_vendidos = 0;
        evento.creador = ctx.accounts.creador.key();

        msg!("✅ Evento '{}' creado con ID: {}", evento.nombre, evento_id);
        Ok(())
    }

    // ==================== 🎫 COMPRAR TICKET (con opción de referral) ====================
    pub fn comprar_ticket(
        ctx: Context<ComprarTicket>,
        evento_id: u64,
        referrer: Option<Pubkey>,
    ) -> Result<()> {
        let evento = &mut ctx.accounts.evento;
        let ticket = &mut ctx.accounts.ticket;
        let usuario_tier = &mut ctx.accounts.usuario_tier;
        let clock = Clock::get()?;

        // Validar capacidad del evento
        require!(
            evento.tickets_vendidos < evento.max_capacidad,
            ErrorCode::EventoLleno
        );

        // Calcular precio con descuento según tier
        let descuento_porcentaje = match usuario_tier.tier {
            2 => 15, // Silver: 15% descuento
            3 => 30, // Gold: 30% descuento
            _ => 0,  // Bronze: sin descuento
        };

        let precio_final = evento.precio_base
            - (evento.precio_base * descuento_porcentaje as u64 / 100);

        // Crear ticket
        ticket.id = evento.tickets_vendidos as u64;
        ticket.evento_id = evento_id;
        ticket.usuario = ctx.accounts.comprador.key();
        ticket.precio_pagado = precio_final;
        ticket.estado = TicketEstado::Activo;
        ticket.fecha_compra = clock.unix_timestamp;
        ticket.fecha_validacion = 0;
        ticket.dentro_horario = false;
        ticket.referrer = referrer;

        // Actualizar datos del evento
        evento.tickets_vendidos += 1;

        // Actualizar tier del usuario (aumentar puntos)
        usuario_tier.puntos_totales += 10; // 10 puntos por compra
        usuario_tier.ultima_compra = clock.unix_timestamp;

        // Si hay referrer, darle bonus
        if let Some(ref_pubkey) = referrer {
            // Aquí iría la lógica para aumentar puntos al referrer
            msg!("🔗 Referral detectado: {}", ref_pubkey);
        }

        msg!(
            "🎫 Ticket comprado por {} | Evento ID: {} | Precio: {} SOL",
            ctx.accounts.comprador.key(),
            evento_id,
            precio_final
        );
        Ok(())
    }

    // ==================== ✅ VALIDAR ASISTENCIA (Escanear QR) ====================
    pub fn validar_asistencia(
        ctx: Context<ValidarAsistencia>,
        evento_id: u64,
    ) -> Result<()> {
        let ticket = &mut ctx.accounts.ticket;
        let usuario_tier = &mut ctx.accounts.usuario_tier;
        let evento = &ctx.accounts.evento;
        let clock = Clock::get()?;

        // Validaciones
        require!(ticket.estado == TicketEstado::Activo, ErrorCode::TicketNoActivo);
        require!(
            ticket.usuario == ctx.accounts.usuario.key(),
            ErrorCode::TicketNoPertenece
        );
        require!(ticket.evento_id == evento_id, ErrorCode::EventoNoCoincide);

        // Verificar si llega a tiempo (30 min antes del evento)
        let ventana_anticipada = 30 * 60; // 30 minutos en segundos
        let dentro_horario = clock.unix_timestamp >= (evento.fecha - ventana_anticipada)
            && clock.unix_timestamp <= evento.fecha;

        // Crear registro de asistencia
        let _asistencia = Asistencia {
            ticket_id: ticket.key(),
            usuario: ctx.accounts.usuario.key(),
            evento_id,
            validado_en: clock.unix_timestamp,
            dentro_horario,
        };

        // Actualizar ticket
        ticket.estado = TicketEstado::Usado;
        ticket.fecha_validacion = clock.unix_timestamp;
        ticket.dentro_horario = dentro_horario;

        // Otorgar puntos según puntualidad
        let puntos_base = 20;
        let puntos_bonus = if dentro_horario { 5 } else { 0 };
        let puntos_totales = puntos_base + puntos_bonus;

        usuario_tier.puntos_totales += puntos_totales;
        usuario_tier.eventos_asistidos += 1;
        usuario_tier.ultimo_evento = clock.unix_timestamp;

        // Actualizar tier automáticamente
        actualizar_tier_automatico(usuario_tier);

        msg!(
            "✅ Asistencia validada | Usuario: {} | Puntos ganados: {} | Tier ahora: {}",
            ctx.accounts.usuario.key(),
            puntos_totales,
            usuario_tier.tier
        );
        Ok(())
    }

    // ==================== 🏆 VER ESTADO DEL TICKET ====================
    pub fn ver_estado_ticket(ctx: Context<VerTicket>) -> Result<()> {
        let ticket = &ctx.accounts.ticket;
        let usuario_tier = &ctx.accounts.usuario_tier;

        msg!("🎫 --- ESTADO DEL TICKET ---");
        msg!("ID: {}", ticket.id);
        msg!("Estado: {:?}", ticket.estado);
        msg!("Precio pagado: {}", ticket.precio_pagado);
        msg!("Dentro de horario: {}", ticket.dentro_horario);
        msg!("👤 --- TIER DEL USUARIO ---");
        msg!("Eventos asistidos: {}", usuario_tier.eventos_asistidos);
        msg!("Puntos totales: {}", usuario_tier.puntos_totales);
        msg!("Tier actual: {}", match usuario_tier.tier {
            1 => "🥉 Bronze",
            2 => "🥈 Silver",
            3 => "🥇 Gold",
            _ => "Sin tier",
        });

        Ok(())
    }

    // ==================== 💰 OBTENER DESCUENTO ====================
    pub fn obtener_descuento(ctx: Context<ObtenerDescuento>) -> Result<u64> {
        let tier = &ctx.accounts.usuario_tier;
        let descuento = match tier.tier {
            1 => 5,   // 5% Bronze
            2 => 15,  // 15% Silver
            3 => 30,  // 30% Gold
            _ => 0,
        };

        msg!("💰 Descuento aplicable: {}%", descuento);
        Ok(descuento)
    }

    // ==================== 📊 VER INVENTARIO DE EVENTOS ====================
    pub fn ver_eventos(ctx: Context<VerEventos>) -> Result<()> {
        let evento = &ctx.accounts.evento;

        msg!("📍 --- EVENTO: {} ---", evento.nombre);
        msg!("ID: {}", evento.id);
        msg!("Fecha: {}", evento.fecha);
        msg!(
            "Tickets: {}/{}",
            evento.tickets_vendidos, evento.max_capacidad
        );
        msg!("Precio base: {} SOL", evento.precio_base);

        Ok(())
    }
}

// ==================== 📌 FUNCIÓN AUXILIAR: ACTUALIZAR TIER ====================
fn actualizar_tier_automatico(usuario_tier: &mut TierUsuario) {
    let nuevo_tier = if usuario_tier.eventos_asistidos >= 5 && usuario_tier.puntos_totales >= 200
    {
        3 // Gold
    } else if usuario_tier.eventos_asistidos >= 3 && usuario_tier.puntos_totales >= 100 {
        2 // Silver
    } else {
        1 // Bronze
    };

    if nuevo_tier > usuario_tier.tier {
        msg!("🎉 ¡ASCENSO! Nuevo tier: {}", nuevo_tier);
    }
    usuario_tier.tier = nuevo_tier;
}

// ==================== 📋 ESTRUCTURAS DE DATOS ====================

#[account]
#[derive(InitSpace)]
pub struct Evento {
    pub id: u64,
    #[max_len(100)]
    pub nombre: String,
    pub fecha: i64,
    pub ubicacion_hash: [u8; 32],
    pub precio_base: u64,
    pub tickets_vendidos: u32,
    pub max_capacidad: u32,
    pub creador: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct Ticket {
    pub id: u64,
    pub evento_id: u64,
    pub usuario: Pubkey,
    pub precio_pagado: u64,
    pub estado: TicketEstado,
    pub fecha_compra: i64,
    pub fecha_validacion: i64,
    pub dentro_horario: bool,
    pub referrer: Option<Pubkey>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace, Debug)]
pub enum TicketEstado {
    Activo,
    Usado,
    Coleccionable,
}

#[account]
#[derive(InitSpace)]
pub struct TierUsuario {
    pub wallet: Pubkey,
    pub eventos_asistidos: u32,
    pub puntos_totales: u64,
    pub tier: u8, // 1: Bronze, 2: Silver, 3: Gold
    pub ultimo_evento: i64,
    pub ultima_compra: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Asistencia {
    pub ticket_id: Pubkey,
    pub usuario: Pubkey,
    pub evento_id: u64,
    pub validado_en: i64,
    pub dentro_horario: bool,
}

// ==================== 🔐 CONTEXTOS DE INSTRUCCIONES ====================

#[derive(Accounts)]
#[instruction(evento_id: u64)]
pub struct CrearEvento<'info> {
    #[account(
        init,
        payer = creador,
        space = 8 + Evento::INIT_SPACE,
        seeds = [b"evento", evento_id.to_le_bytes().as_ref()],
        bump
    )]
    pub evento: Account<'info, Evento>,
    #[account(mut)]
    pub creador: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(evento_id: u64)]
pub struct ComprarTicket<'info> {
    #[account(mut, seeds = [b"evento", evento_id.to_le_bytes().as_ref()], bump)]
    pub evento: Account<'info, Evento>,
    #[account(
        init,
        payer = comprador,
        space = 8 + Ticket::INIT_SPACE,
        seeds = [b"ticket", evento_id.to_le_bytes().as_ref(), comprador.key().as_ref()],
        bump
    )]
    pub ticket: Account<'info, Ticket>,
    #[account(
        init_if_needed,
        payer = comprador,
        space = 8 + TierUsuario::INIT_SPACE,
        seeds = [b"tier", comprador.key().as_ref()],
        bump
    )]
    pub usuario_tier: Account<'info, TierUsuario>,
    #[account(mut)]
    pub comprador: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ValidarAsistencia<'info> {
    #[account(mut)]
    pub ticket: Account<'info, Ticket>,
    #[account(mut)]
    pub usuario_tier: Account<'info, TierUsuario>,
    pub evento: Account<'info, Evento>,
    pub usuario: Signer<'info>,
}

#[derive(Accounts)]
pub struct VerTicket<'info> {
    pub ticket: Account<'info, Ticket>,
    pub usuario_tier: Account<'info, TierUsuario>,
}

#[derive(Accounts)]
pub struct ObtenerDescuento<'info> {
    pub usuario_tier: Account<'info, TierUsuario>,
}

#[derive(Accounts)]
pub struct VerEventos<'info> {
    pub evento: Account<'info, Evento>,
}

// ==================== ⚠️ ERRORES ====================

#[error_code]
pub enum ErrorCode {
    #[msg("El evento ya está lleno")]
    EventoLleno,
    #[msg("El ticket no está activo")]
    TicketNoActivo,
    #[msg("El ticket no pertenece a este usuario")]
    TicketNoPertenece,
    #[msg("El evento no coincide")]
    EventoNoCoincide,
}
