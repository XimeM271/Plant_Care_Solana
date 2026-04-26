use anchor_lang::prelude::*;

// Puedes reemplazar este ID con el que te genere Solana Playground
declare_id!("HYVAtyayLC2hPw96gCuHu1DZA4krAHmvmZwqbN4xFkqz");

#[program]
pub mod plant_care_solana {
    use super::*;

    pub fn inicializar_invernadero(ctx: Context<CrearInvernadero>, nombre: String) -> Result<()> {
        let invernadero = &mut ctx.accounts.invernadero;
        invernadero.owner = ctx.accounts.owner.key();
        invernadero.nombre_invernadero = nombre;
        invernadero.plantas = Vec::new(); 
        
        msg!("Invernadero '{}' creado", invernadero.nombre_invernadero);
        Ok(())
    }

    pub fn registrar_planta(
        ctx: Context<GestionarInvernadero>, 
        especie: String, 
        volumen_ml: u16, 
        frecuencia_dias: u8,
        luz_directa: bool
    ) -> Result<()> {
        let invernadero = &mut ctx.accounts.invernadero;
        
        require!(invernadero.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);

        let nueva_planta = Planta {
            especie,
            volumen_ml,
            frecuencia_dias,
            luz_directa,
        };

        invernadero.plantas.push(nueva_planta);
        msg!("Planta registrada.");
        Ok(())
    }

    pub fn editar_planta(
        ctx: Context<GestionarInvernadero>, 
        especie: String, 
        nuevo_volumen: u16, 
        nueva_frecuencia: u8,
        nueva_luz_directa: bool
    ) -> Result<()> {
        let invernadero = &mut ctx.accounts.invernadero;
        require!(invernadero.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);

        let lista = &mut invernadero.plantas;
        for i in 0..lista.len() {
            if lista[i].especie == especie {
                lista[i].volumen_ml = nuevo_volumen;
                lista[i].frecuencia_dias = nueva_frecuencia;
                lista[i].luz_directa = nueva_luz_directa;
                msg!("Cuidados de la planta actualizados.");
                return Ok(());
            }
        }
        Err(Errores::PlantaNoExiste.into())
    }

    pub fn eliminar_planta(ctx: Context<GestionarInvernadero>, especie: String) -> Result<()> {
        let invernadero = &mut ctx.accounts.invernadero;
        require!(invernadero.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);

        let lista = &mut invernadero.plantas;
        let index = lista.iter().position(|x| x.especie == especie);
        
        if let Some(i) = index {
            lista.remove(i);
            msg!("Planta eliminada del invernadero.");
            Ok(())
        } else {
            Err(Errores::PlantaNoExiste.into())
        }
    }

    pub fn leer_invernadero(ctx: Context<GestionarInvernadero>) -> Result<()> {
        msg!("Invernadero: {}", ctx.accounts.invernadero.nombre_invernadero);
        msg!("Datos: {:#?}", ctx.accounts.invernadero.plantas);
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Planta {
    #[max_len(30)]
    pub especie: String,
    pub volumen_ml: u16,
    pub frecuencia_dias: u8,
    pub luz_directa: bool,
}

#[account]
#[derive(InitSpace)]
pub struct Invernadero {
    pub owner: Pubkey,
    #[max_len(40)]
    pub nombre_invernadero: String,
    #[max_len(15)] 
    pub plantas: Vec<Planta>,
}

#[derive(Accounts)]
pub struct CrearInvernadero<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + Invernadero::INIT_SPACE,
        seeds = [b"invernadero", owner.key().as_ref()],
        bump
    )]
    pub invernadero: Account<'info, Invernadero>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GestionarInvernadero<'info> {
    pub owner: Signer<'info>,
    #[account(mut)] 
    pub invernadero: Account<'info, Invernadero>,
}

#[error_code]
pub enum Errores {
    #[msg("No tienes permisos.")]
    NoEresElOwner,
    #[msg("La planta no existe en el invernadero.")]
    PlantaNoExiste,
}
