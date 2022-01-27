macro_rules! generate {
	{
		Realm::Server => {
			$($sv_name:ident: { $sv_from:literal, $sv_to:literal }),*
		},
		Realm::Client => {
			$($cl_name:ident: { $cl_from:literal, $cl_to:literal }),*
		}
	} => {
		mod server {
			$(
				#[test]
				fn $sv_name() {
					crate::init();
					crate::enums::test_data();

					let mut src = $sv_from.trim().as_bytes().to_vec();
					let strip_tree = crate::strip::generate(&src);
					crate::realms::optimise(crate::realms::Realm::Server, &mut src, &strip_tree);
					crate::enums::optimise(&mut src, &strip_tree);

					let a = String::from_utf8_lossy(&src);
					let b = $sv_to.trim();
					if src.as_slice() != b.as_bytes() {
						panic!("Mismatch\n\n== Left ==\n{}\n\n== Right ==\n{}", a, b);
					}
				}
			)*
		}
		mod client {
			$(
				#[test]
				fn $cl_name() {
					crate::init();
					crate::enums::test_data();

					let mut src = $cl_from.trim().as_bytes().to_vec();
					let strip_tree = crate::strip::generate(&src);
					crate::realms::optimise(crate::realms::Realm::Client, &mut src, &strip_tree);
					crate::enums::optimise(&mut src, &strip_tree);

					let a = String::from_utf8_lossy(&src);
					let b = $cl_to.trim();
					if src.as_slice() != b.as_bytes() {
						panic!("Mismatch\n\n== Left ==\n{}\n\n== Right ==\n{}", a, b);
					}
				}
			)*
		}
	};
}

generate! {
	Realm::Server => {
		localizations: {
			r#"
			(local SERVER = SERVER)
			(local CLIENT = CLIENT)
			(local ACT_INVALID = ACT_INVALID)
			"#,

			r#"
			(local SERVER = true  )
			(local CLIENT = false )
			(local ACT_INVALID = -1         )
			"#
		},

		enums: {
			r#"
			(ACT_INVALID)
			(ACT_RESET)
			(ACT_IDLE)
			(ACT_TRANSITION)
			(ACT_COVER)
			(ACT_COVER_MED)
			(ACT_COVER_LOW)
			(ACT_WALK)
			(ACT_WALK_AIM)
			(ACT_WALK_CROUCH)
			(ACT_WALK_CROUCH_AIM)
			(ACT_RUN)
			(ACT_RUN_AIM)
			(ACT_RUN_CROUCH)
			(ACT_RUN_CROUCH_AIM)
			"#,

			r#"
			(-1         )
			(0        )
			(1       )
			(2             )
			(3        )
			(4            )
			(5            )
			(6       )
			(7           )
			(8              )
			(9                  )
			(10     )
			(11         )
			(12            )
			(13                )
			"#
		},

		server_and_client: {
			r#"
			-- SERVER
			local _ = "SERVER CLIENT"
			local _ = "SER\"VER"
			local _ = "SER\\\"VER"
			local _ = 'SER\'VER'
			local _ = [[SERVER]]
			local _ = [[SER]\]VER]]
			local _ = [[SER\]\]VER]]
			local _ = [[SERVER CLIENT
			SERVER]]
			local _ = "" .. SERVER .. ""

			-- CLIENT
			local _ = "CLIENT SERVER"
			local _ = "CLIENT\"CLIENT"
			local _ = "CLIENT\\\"CLIENT"
			local _ = 'CLIENT\'CLIENT'
			local _ = [[CLIENT]]
			local _ = [[CLIENT]\]CLIENT]]
			local _ = [[CLIENT\]\]CLIENT]]
			local _ = [[CLIENT SERVER
			CLIENT]]
			local _ = "" .. CLIENT .. ""

			--[[
				multiline comment
				SERVER
				CLIENT
			]]

			--[=[
				multiline comment
				SERVER
				CLIENT
			]=]

			--[==========[
				multiline comment
				SERVER
				CLIENT
			]==========]

			if SERVER and not lol or not hi and SERVER and CLIENT then

			end
			"#,

			r#"
			-- SERVER
			local _ = "SERVER CLIENT"
			local _ = "SER\"VER"
			local _ = "SER\\\"VER"
			local _ = 'SER\'VER'
			local _ = [[SERVER]]
			local _ = [[SER]\]VER]]
			local _ = [[SER\]\]VER]]
			local _ = [[SERVER CLIENT
			SERVER]]
			local _ = "" .. true   .. ""

			-- CLIENT
			local _ = "CLIENT SERVER"
			local _ = "CLIENT\"CLIENT"
			local _ = "CLIENT\\\"CLIENT"
			local _ = 'CLIENT\'CLIENT'
			local _ = [[CLIENT]]
			local _ = [[CLIENT]\]CLIENT]]
			local _ = [[CLIENT\]\]CLIENT]]
			local _ = [[CLIENT SERVER
			CLIENT]]
			local _ = "" .. false  .. ""

			--[[
				multiline comment
				SERVER
				CLIENT
			]]

			--[=[
				multiline comment
				SERVER
				CLIENT
			]=]

			--[==========[
				multiline comment
				SERVER
				CLIENT
			]==========]

			if true   and not lol or not hi and true   and false  then

			end
			"#
		}
	},

	Realm::Client => {
		localizations: {
			r#"
			(local SERVER = SERVER)
			(local CLIENT = CLIENT)
			(local ACT_INVALID = ACT_INVALID)
			"#,

			r#"
			(local SERVER = false )
			(local CLIENT = true  )
			(local ACT_INVALID = -1         )
			"#
		},

		enums: {
			r#"
			(ACT_INVALID)
			(ACT_RESET)
			(ACT_IDLE)
			(ACT_TRANSITION)
			(ACT_COVER)
			(ACT_COVER_MED)
			(ACT_COVER_LOW)
			(ACT_WALK)
			(ACT_WALK_AIM)
			(ACT_WALK_CROUCH)
			(ACT_WALK_CROUCH_AIM)
			(ACT_RUN)
			(ACT_RUN_AIM)
			(ACT_RUN_CROUCH)
			(ACT_RUN_CROUCH_AIM)
			"#,

			r#"
			(-1         )
			(0        )
			(1       )
			(2             )
			(3        )
			(4            )
			(5            )
			(6       )
			(7           )
			(8              )
			(9                  )
			(10     )
			(11         )
			(12            )
			(13                )
			"#
		},

		server_and_client: {
			r#"
			-- SERVER
			local _ = "SERVER CLIENT"
			local _ = "SERVER\"SERVER"
			local _ = "SERVER\\\"SERVER"
			local _ = 'SERVER\'SERVER'
			local _ = [[SERVER]]
			local _ = [[SERVER]\]SERVER]]
			local _ = [[SERVER\]\]SERVER]]
			local _ = [[SERVER CLIENT
			SERVER]]
			local _ = "" .. SERVER .. ""

			-- CLIENT
			local _ = "CLIENT SERVER"
			local _ = "CLIENT\"CLIENT"
			local _ = "CLIENT\\\"CLIENT"
			local _ = 'CLIENT\'CLIENT'
			local _ = [[CLIENT]]
			local _ = [[CLIENT]\]CLIENT]]
			local _ = [[CLIENT\]\]CLIENT]]
			local _ = [[CLIENT SERVER
			CLIENT]]
			local _ = "" .. CLIENT .. ""

			--[[
				multiline comment
				SERVER
				CLIENT
			]]

			--[=[
				multiline comment
				SERVER
				CLIENT
			]=]

			--[==========[
				multiline comment
				SERVER
				CLIENT
			]==========]

			if SERVER and not lol or not hi and SERVER and CLIENT then

			end
			"#,

			r#"
			-- SERVER
			local _ = "SERVER CLIENT"
			local _ = "SERVER\"SERVER"
			local _ = "SERVER\\\"SERVER"
			local _ = 'SERVER\'SERVER'
			local _ = [[SERVER]]
			local _ = [[SERVER]\]SERVER]]
			local _ = [[SERVER\]\]SERVER]]
			local _ = [[SERVER CLIENT
			SERVER]]
			local _ = "" .. false  .. ""

			-- CLIENT
			local _ = "CLIENT SERVER"
			local _ = "CLIENT\"CLIENT"
			local _ = "CLIENT\\\"CLIENT"
			local _ = 'CLIENT\'CLIENT'
			local _ = [[CLIENT]]
			local _ = [[CLIENT]\]CLIENT]]
			local _ = [[CLIENT\]\]CLIENT]]
			local _ = [[CLIENT SERVER
			CLIENT]]
			local _ = "" .. true   .. ""

			--[[
				multiline comment
				SERVER
				CLIENT
			]]

			--[=[
				multiline comment
				SERVER
				CLIENT
			]=]

			--[==========[
				multiline comment
				SERVER
				CLIENT
			]==========]

			if false  and not lol or not hi and false  and true   then

			end
			"#
		}
	}
}