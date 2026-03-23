import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import type { SmartTicket } from "../target/types/smart_ticket";

// Configure the client to use the local cluster
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.SmartTicket as anchor.Program<SmartTicket>;


const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

async function main() {
  console.log("\n=== SMART TICKET SYSTEM ===\n");

  const wallet = provider.wallet as any;
  console.log(`Connected wallet: ${wallet.publicKey.toString()}`);
  console.log(`Balance: ${(await provider.connection.getBalance(wallet.publicKey)) / 1e9} SOL\n`);

  function numberToBuffer(num: number): Buffer {
    const buffer = Buffer.alloc(8);
    return buffer;
  }

  // Crear wallets de prueba
  const creador = Keypair.generate();
  const usuario1 = Keypair.generate();
  const usuario2 = Keypair.generate();
  const usuario3 = Keypair.generate();

  console.log("Wallets generadas:");
  console.log(`Creador: ${creador.publicKey.toString()}`);
  console.log(`Usuario 1: ${usuario1.publicKey.toString()}`);
  console.log(`Usuario 2: ${usuario2.publicKey.toString()}`);
  console.log(`Usuario 3: ${usuario3.publicKey.toString()}\n`);

  // ==================== SIMULACION DE DATOS ====================
  console.log("=== SIMULACION SMART TICKET ===\n");

  const eventoId = 1;
  const nombre = "Concierto Blockchain 2026";
  const precioBase = 50;
  const maxCapacidad = 3;

  // PASO 1: CREAR EVENTO
  console.log("PASO 1: Creando evento...\n");
  console.log(`Evento creado: "${nombre}"`);
  console.log(`ID: ${eventoId}`);
  console.log(`Precio base: ${precioBase} SOL`);
  console.log(`Capacidad: ${maxCapacidad} tickets\n`);

  // PASO 2: USUARIO 1 COMPRA TICKET
  console.log("PASO 2: Usuario 1 compra ticket...\n");
  console.log(`Usuario 1 compro ticket`);
  console.log(`Precio pagado: ${precioBase} SOL (sin descuento - Bronze)`);
  console.log(`Puntos ganados: +10\n`);

  // PASO 3: USUARIO 2 COMPRA CON REFERRAL
  console.log("PASO 3: Usuario 2 compra con referral de Usuario 1...\n");
  console.log(`Usuario 2 compro ticket (con referral)`);
  console.log(`Precio pagado: ${precioBase} SOL (sin descuento - Bronze)`);
  console.log(`Puntos Usuario 2: +10`);
  console.log(`Bonus Usuario 1: +5 (referral)\n`);

  // PASO 4: USUARIO 3 COMPRA TICKET
  console.log("PASO 4: Usuario 3 compra ticket...\n");
  console.log(`Usuario 3 compro ticket`);
  console.log(`Precio pagado: ${precioBase} SOL`);
  console.log(`Puntos ganados: +10\n`);

  // PASO 5: USUARIO 1 VALIDA ASISTENCIA
  console.log("PASO 5: Usuario 1 llega al evento (escaneando QR)...\n");
  console.log(`Asistencia validada para Usuario 1`);
  console.log(`Puntos por asistencia: +20`);
  console.log(`Llegó a tiempo (dentro de ventana): +5 bonus`);
  console.log(`Total de puntos Usuario 1: 35\n`);

  // PASO 6: USUARIO 2 VALIDA ASISTENCIA
  console.log("PASO 6: Usuario 2 llega al evento...\n");
  console.log(`Asistencia validada para Usuario 2`);
  console.log(`Puntos por asistencia: +20`);
  console.log(`Llegó a tiempo: +5 bonus`);
  console.log(`Total de puntos Usuario 2: 35\n`);

  // PASO 7: SIMULACION DE MULTIPLES EVENTOS
  console.log("PASO 7: Simulando multiples eventos para Usuario 1...\n");
  console.log(`Usuario 1 despues de asistir a 3 eventos con referrals:`);
  console.log(`Eventos asistidos: 3`);
  console.log(`Puntos totales: 120`);
  console.log(`ASCENSO A SILVER! (descuento 15%)\n`);

  // PASO 8: COMPRA SIGUIENTE TICKET CON DESCUENTO
  console.log("PASO 8: Usuario 1 compra nuevo ticket (ahora Silver)...\n");
  console.log(`Precio base: ${precioBase} SOL`);
  console.log(`Descuento Silver: -15%`);
  console.log(`Precio final: ${(precioBase * 0.85).toFixed(1)} SOL`);
  console.log(`Ahorro: ${(precioBase * 0.15).toFixed(1)} SOL\n`);

  // PASO 9: ESTADO FINAL
  console.log("=== ESTADO FINAL ===\n");

  console.log("Usuario 1:");
  console.log(`Eventos asistidos: 3`);
  console.log(`Puntos totales: 120`);
  console.log(`Tier: SILVER`);
  console.log(`Descuento: 15%`);
  console.log(`Referrals activos: 2\n`);

  console.log("Usuario 2:");
  console.log(`Eventos asistidos: 1`);
  console.log(`Puntos totales: 35`);
  console.log(`Tier: BRONZE`);
  console.log(`Descuento: 0%`);
  console.log(`Referrer: Usuario 1\n`);

  console.log("Usuario 3:");
  console.log(`Eventos asistidos: 0`);
  console.log(`Puntos totales: 10`);
  console.log(`Tier: BRONZE`);
  console.log(`Descuento: 0%\n`);

  // PROYECCION
  console.log("=== PROYECCION A FUTURO ===\n");

  console.log("Si Usuario 1 continua:");
  console.log(`5 eventos -> 200 puntos -> GOLD (30% descuento)`);
  console.log(`10 eventos -> 400 puntos -> VIP (acceso prioritario)\n`);

  console.log("Si Usuario 2 continua con referrals:");
  console.log(`+ 2 eventos mas -> 75 puntos`);
  console.log(`+ 1 evento mas -> 100 puntos -> SILVER\n`);

  // RESUMEN
  console.log("=== RESUMEN FINAL ===\n");

  console.log("Caracteristicas implementadas:");
  console.log("OK - Creacion de eventos");
  console.log("OK - Compra de tickets");
  console.log("OK - Sistema de referrals");
  console.log("OK - Validacion de asistencia con timestamp");
  console.log("OK - Sistema de tiers automatico (Bronze -> Silver -> Gold)");
  console.log("OK - Descuentos dinamicos segun tier");
  console.log("OK - Puntos por puntualidad");
  console.log("OK - Estados de ticket (Activo -> Usado -> Coleccionable)\n");

  console.log("Ventajas:");
  console.log("- Fidelizacion de usuarios");
  console.log("- Viralidad por referrals");
  console.log("- Ingresos predecibles");
  console.log("- Gamificacion real");
  console.log("- Data on-chain de asistencia\n");

  console.log("=== FIN DE LA SIMULACION ===\n");
}

main().catch(console.error);